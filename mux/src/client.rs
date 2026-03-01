use crate::PaneId;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

static CLIENT_ID: AtomicUsize = AtomicUsize::new(0);
static EPOCH: std::sync::LazyLock<u64> = std::sync::LazyLock::new(|| {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientId {
    pub hostname: String,
    pub username: String,
    pub pid: u32,
    pub epoch: u64,
    pub id: usize,
    pub ssh_auth_sock: Option<String>,
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientId {
    pub fn new() -> Self {
        let id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            hostname: hostname::get().map_or_else(
                |_| "localhost".to_string(),
                |s| s.to_string_lossy().to_string(),
            ),
            username: config::username_from_env().unwrap_or_else(|_| "somebody".to_string()),
            pid: unsafe { libc::getpid() as u32 },
            epoch: *EPOCH,
            id,
            ssh_auth_sock: crate::AgentProxy::default_ssh_auth_sock(),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct ClientInfo {
    pub client_id: Arc<ClientId>,
    /// The time this client last connected
    #[serde(with = "ts_seconds")]
    pub connected_at: DateTime<Utc>,
    /// Which workspace is active
    pub active_workspace: Option<String>,
    /// The last time we received input from this client
    #[serde(with = "ts_seconds")]
    pub last_input: DateTime<Utc>,
    /// The currently-focused pane
    pub focused_pane_id: Option<PaneId>,
}

impl ClientInfo {
    #[must_use]
    pub fn new(client_id: Arc<ClientId>) -> Self {
        Self {
            client_id,
            connected_at: Utc::now(),
            active_workspace: None,
            last_input: Utc::now(),
            focused_pane_id: None,
        }
    }

    pub fn update_last_input(&mut self) {
        self.last_input = Utc::now();
    }

    pub const fn update_focused_pane(&mut self, pane_id: PaneId) {
        self.focused_pane_id.replace(pane_id);
    }
}
