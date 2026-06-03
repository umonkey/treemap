use crate::services::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_lab::sse;
use futures::StreamExt;
use log::info;
use std::time::Duration;

#[get("")]
pub async fn sse_handler(state: web::Data<AppState>, _req: HttpRequest) -> impl Responder {
    let (id, rx) = state.mcp.create_session().await;

    info!("New MCP session started: {}", id);

    let post_url = format!(
        "http://{}/mcp/message?session_id={}",
        _req.connection_info().host(),
        id
    );
    let mut data = sse::Data::new(post_url);
    data.set_event("endpoint");
    let initial_event = sse::Event::Data(data);

    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
        .map(|msg| Ok::<_, actix_web::Error>(sse::Event::Data(sse::Data::new(msg))));

    let full_stream =
        futures::stream::once(async move { Ok::<_, actix_web::Error>(initial_event) })
            .chain(stream);

    sse::Sse::from_stream(full_stream).with_keep_alive(Duration::from_secs(15))
}
