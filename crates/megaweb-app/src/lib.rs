pub mod components;
pub mod pages;
pub mod state;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use pages::{
    connections::ConnectionsPage, k8s_dashboard::K8sDashboardPage, monitoring::MonitoringPage,
    schema_browser::SchemaBrowserPage, settings::SettingsPage, sql_editor::SqlEditorPage,
};

use components::sidebar::Sidebar;

/// Root application shell with router.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="app-styles" href="/style/main.css" />
        <Title text="MegaDB Web" />
        <Meta name="description" content="Web Client for MegaDB — SQL Editor, K8s Dashboard, Performance Monitoring" />

        <Router>
            <div class="app-layout">
                <Sidebar />
                <main class="app-main">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("/") view=SqlEditorPage />
                        <Route path=path!("/sql") view=SqlEditorPage />
                        <Route path=path!("/schema") view=SchemaBrowserPage />
                        <Route path=path!("/k8s") view=K8sDashboardPage />
                        <Route path=path!("/monitoring") view=MonitoringPage />
                        <Route path=path!("/connections") view=ConnectionsPage />
                        <Route path=path!("/settings") view=SettingsPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <p>"Page not found"</p>
            <a href="/">"Go to SQL Editor"</a>
        </div>
    }
}
