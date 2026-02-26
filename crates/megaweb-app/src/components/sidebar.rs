use leptos::prelude::*;

/// Main navigation sidebar.
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="sidebar">
            <div class="sidebar-header">
                <h1 class="sidebar-logo">"MegaDB"</h1>
                <span class="sidebar-version">"v0.1"</span>
            </div>

            <div class="sidebar-nav">
                <SidebarLink href="/sql" icon="terminal" label="SQL Editor" />
                <SidebarLink href="/schema" icon="database" label="Schema" />
                <SidebarLink href="/k8s" icon="server" label="Kubernetes" />
                <SidebarLink href="/monitoring" icon="activity" label="Monitoring" />
            </div>

            <div class="sidebar-footer">
                <SidebarLink href="/connections" icon="plug" label="Connections" />
                <SidebarLink href="/settings" icon="settings" label="Settings" />
            </div>
        </nav>
    }
}

#[component]
fn SidebarLink(href: &'static str, icon: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a href=href class="sidebar-link">
            <span class="sidebar-link-icon">{icon_svg(icon)}</span>
            <span class="sidebar-link-label">{label}</span>
        </a>
    }
}

/// Simple text icons for sidebar navigation.
fn icon_svg(name: &str) -> &'static str {
    match name {
        "terminal" => ">_",
        "database" => "DB",
        "server" => "K8",
        "activity" => "PM",
        "plug" => "CN",
        "settings" => "ST",
        _ => "??",
    }
}
