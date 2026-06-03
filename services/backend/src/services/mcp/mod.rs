use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub mod schemas;
pub mod service;

pub use service::McpService;

pub type McpSender = mpsc::UnboundedSender<String>;

pub struct McpSession {
    #[allow(dead_code)]
    pub id: Uuid,
    pub tx: McpSender,
}

#[derive(Default)]
pub struct McpSessionManager {
    sessions: Mutex<HashMap<Uuid, McpSession>>,
}

impl McpSessionManager {
    pub async fn create_session(&self) -> (Uuid, mpsc::UnboundedReceiver<String>) {
        let id = Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();
        let session = McpSession { id, tx };

        let mut sessions = self.sessions.lock().await;
        sessions.insert(id, session);

        (id, rx)
    }

    pub async fn get_sender(&self, id: Uuid) -> Option<McpSender> {
        let sessions = self.sessions.lock().await;
        sessions.get(&id).map(|s| s.tx.clone())
    }

    #[allow(dead_code)]
    pub async fn remove_session(&self, id: Uuid) {
        let mut sessions = self.sessions.lock().await;
        sessions.remove(&id);
    }
}
