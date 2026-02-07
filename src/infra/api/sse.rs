//! Server-Sent Events (SSE) for real-time updates.
//!
//! Broadcasts match updates, bracket changes, and notifications to connected clients.
//! Connect via `GET /events` for the event stream.

use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web::web;
use actix_web_lab::sse::{self, Sse};
use futures_util::future;
use futures_util::StreamExt;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Event types for real-time updates
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeEvent {
    MatchUpdate {
        match_id: uuid::Uuid,
        tournament_id: Option<uuid::Uuid>,
        category_id: Option<uuid::Uuid>,
        status: Option<String>,
    },
    BracketUpdate {
        tournament_id: uuid::Uuid,
        category_id: Option<uuid::Uuid>,
    },
    Notification {
        user_id: uuid::Uuid,
        notification_id: uuid::Uuid,
    },
}

impl RealtimeEvent {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<mpsc::Sender<sse::Event>>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Self::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 15 seconds to remove stale connections.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(15));
            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers a new SSE client. Returns the SSE response body.
    pub async fn new_client(&self) -> impl actix_web::Responder {
        let (tx, rx) = mpsc::channel(32);

        let _ = tx
            .send(sse::Data::new("connected").into())
            .await;

        self.inner.lock().clients.push(tx);

        let stream = ReceiverStream::new(rx).map(|e| Ok::<_, std::convert::Infallible>(e));
        Sse::from_stream(stream)
    }

    /// Broadcasts an event to all connected clients.
    pub async fn broadcast_event(&self, event: &RealtimeEvent) {
        let data = event.to_json();
        self.broadcast(&data).await;
    }

    /// Broadcasts a raw message to all connected clients.
    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .map(|client| client.send(sse::Data::new(msg).into()));

        let _ = future::join_all(send_futures).await;
    }
}

/// SSE handler for GET /events - streams real-time updates to clients.
pub async fn event_stream(
    broadcaster: web::Data<Arc<Broadcaster>>,
) -> impl actix_web::Responder {
    broadcaster.new_client().await
}
