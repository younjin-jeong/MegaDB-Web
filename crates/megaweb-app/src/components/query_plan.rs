use leptos::prelude::*;

/// Placeholder for EXPLAIN query plan visualization.
/// Will be implemented in Phase 5 with tree/graph rendering.
#[component]
pub fn QueryPlanViewer(plan_text: Signal<Option<String>>) -> impl IntoView {
    view! {
        <div class="query-plan">
            {move || match plan_text.get() {
                None => view! {
                    <p class="query-plan-empty">"Run EXPLAIN to see query plan"</p>
                }.into_any(),
                Some(text) => view! {
                    <pre class="query-plan-text">{text}</pre>
                }.into_any(),
            }}
        </div>
    }
}
