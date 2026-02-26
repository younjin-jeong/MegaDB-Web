use leptos::prelude::*;

/// CodeMirror 6 SQL editor wrapper.
///
/// This component creates a `<div>` element and, on the client side,
/// initializes a CodeMirror 6 editor instance via JS interop.
/// On the server (SSR), it renders a simple `<textarea>` fallback.
#[component]
pub fn CodeMirrorEditor(
    /// Initial SQL content.
    #[prop(default = String::new())]
    initial_content: String,
    /// Callback when user triggers "execute" (Ctrl+Enter).
    #[prop(into)]
    on_execute: Callback<String>,
    /// Callback when content changes.
    #[prop(into, optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let (content, set_content) = signal(initial_content.clone());

    // SSR fallback: render a textarea. On client, JS will replace this with CodeMirror.
    view! {
        <div class="codemirror-wrapper" data-editor="sql">
            <textarea
                class="codemirror-fallback"
                prop:value=move || content.get()
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    set_content.set(val.clone());
                    if let Some(cb) = &on_change {
                        cb.run(val);
                    }
                }
                on:keydown=move |ev| {
                    // Ctrl+Enter or Cmd+Enter to execute
                    if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "Enter" {
                        ev.prevent_default();
                        on_execute.run(content.get());
                    }
                }
                placeholder="-- Write your SQL here...\n-- Press Ctrl+Enter to execute"
                rows=12
                spellcheck="false"
            ></textarea>
        </div>
    }
}
