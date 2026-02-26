use leptos::prelude::*;

/// Placeholder chart component.
/// Will integrate Charming (Apache ECharts) in Phase 4.
#[component]
pub fn Chart(title: String, #[prop(default = 300)] height: u32) -> impl IntoView {
    view! {
        <div class="chart-container" style=format!("height:{height}px")>
            <div class="chart-title">{title}</div>
            <div class="chart-placeholder">
                <p>"Chart will be rendered here"</p>
            </div>
        </div>
    }
}
