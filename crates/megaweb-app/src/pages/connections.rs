use leptos::prelude::*;
use megaweb_types::connection::ConnectionConfig;

/// Connection Manager page.
#[component]
pub fn ConnectionsPage() -> impl IntoView {
    let (connections, set_connections) = signal(vec![ConnectionConfig::default()]);
    let (_editing, set_editing) = signal(Option::<usize>::None);

    view! {
        <div class="connections-page">
            <h2>"Connections"</h2>

            <div class="connections-list">
                {move || {
                    connections.get().iter().enumerate().map(|(i, conn)| {
                        let name = conn.name.clone();
                        let host = conn.host.clone();
                        let port = conn.http_port;
                        view! {
                            <div class="connection-card">
                                <div class="connection-info">
                                    <span class="connection-name">{name}</span>
                                    <span class="connection-host">{format!("{host}:{port}")}</span>
                                </div>
                                <div class="connection-actions">
                                    <button class="btn btn-sm btn-primary">"Connect"</button>
                                    <button
                                        class="btn btn-sm"
                                        on:click=move |_| set_editing.set(Some(i))
                                    >
                                        "Edit"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <button
                class="btn btn-secondary"
                on:click=move |_| {
                    set_connections.update(|c| c.push(ConnectionConfig::default()));
                }
            >
                "+ Add Connection"
            </button>
        </div>
    }
}
