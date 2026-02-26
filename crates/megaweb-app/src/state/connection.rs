use leptos::prelude::*;
use megaweb_types::connection::{ConnectionConfig, ConnectionStatus};

/// Global connection state.
#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub active: Option<ConnectionConfig>,
    pub status: ConnectionStatus,
    pub saved_connections: Vec<ConnectionConfig>,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            active: Some(ConnectionConfig::default()),
            status: ConnectionStatus::Disconnected,
            saved_connections: vec![ConnectionConfig::default()],
        }
    }
}

pub fn provide_connection_state() {
    let state = signal(ConnectionState::default());
    provide_context(state);
}

pub fn use_connection_state() -> (ReadSignal<ConnectionState>, WriteSignal<ConnectionState>) {
    expect_context::<(ReadSignal<ConnectionState>, WriteSignal<ConnectionState>)>()
}
