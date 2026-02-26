use axum::Router;
use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

mod config;
mod proxy;
mod websocket;

fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options />
                <link rel="stylesheet" href="/style/main.css" />
            </head>
            <body>
                <megaweb_app::App />
            </body>
        </html>
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load config
    let app_config = config::AppConfig::from_env();
    tracing::info!("Starting MegaDB Web on {}", app_config.bind_address);
    tracing::info!("MegaDB backend: {}", app_config.megadb_url);

    // Leptos configuration
    let leptos_options = LeptosOptions::builder()
        .output_name("megaweb")
        .site_root("site")
        .site_pkg_dir("pkg")
        .site_addr(
            app_config
                .bind_address
                .parse::<std::net::SocketAddr>()
                .unwrap_or_else(|_| std::net::SocketAddr::from(([0, 0, 0, 0], 3000))),
        )
        .build();

    let routes = generate_route_list(megaweb_app::App);

    // CORS layer for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the Axum router with Leptos routes.
    // Router<LeptosOptions> is required for LeptosRoutes trait.
    let app: Router<LeptosOptions> = Router::default();
    let app = app
        .leptos_routes(&leptos_options, routes, {
            let options = leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(cors)
        .with_state(leptos_options);

    // Start server
    let listener = tokio::net::TcpListener::bind(&app_config.bind_address)
        .await
        .expect("Failed to bind");
    tracing::info!("Listening on http://{}", app_config.bind_address);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server error");
}
