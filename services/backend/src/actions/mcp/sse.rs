use crate::services::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_lab::sse;
use futures::StreamExt;
use log::info;
use std::future::ready;
use std::time::Duration;
use uuid::Uuid;

struct SseCleanupStream<S> {
    stream: S,
    state: web::Data<AppState>,
    session_id: Uuid,
}

impl<S: futures::Stream + Unpin> futures::Stream for SseCleanupStream<S> {
    type Item = S::Item;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.stream).poll_next(cx)
    }
}

impl<S> Drop for SseCleanupStream<S> {
    fn drop(&mut self) {
        let state = self.state.clone();
        let id = self.session_id;
        tokio::spawn(async move {
            state.mcp.remove_session(id).await;
            info!("MCP session ended: {}", id);
        });
    }
}

#[get("")]
pub async fn sse_handler(state: web::Data<AppState>, _req: HttpRequest) -> impl Responder {
    let (id, rx) = state.mcp.create_session().await;

    info!("New MCP session started: {}", id);

    let post_url = format!(
        "{}://{}/mcp/message?session_id={}",
        _req.connection_info().scheme(),
        _req.connection_info().host(),
        id
    );
    let mut data = sse::Data::new(post_url);
    data.set_event("endpoint");
    let initial_event = sse::Event::Data(data);

    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx).map(|msg| {
        let mut data = sse::Data::new(msg);
        data.set_event("message");
        Ok::<_, actix_web::Error>(sse::Event::Data(data))
    });

    let full_stream =
        futures::stream::once(ready(Ok::<_, actix_web::Error>(initial_event))).chain(stream);

    let cleanup_stream = SseCleanupStream {
        stream: full_stream,
        state: state.clone(),
        session_id: id,
    };

    sse::Sse::from_stream(cleanup_stream)
        .with_keep_alive(Duration::from_secs(15))
        .customize()
        .insert_header(("content-type", "text/event-stream"))
}
