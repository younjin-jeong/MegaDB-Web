use leptos::prelude::*;

/// Toast notification type.
#[derive(Debug, Clone, PartialEq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// A single toast notification.
#[derive(Debug, Clone)]
pub struct Toast {
    pub id: uuid::Uuid,
    pub message: String,
    pub level: ToastLevel,
}

/// Toast notification container.
#[component]
pub fn ToastContainer(toasts: Signal<Vec<Toast>>) -> impl IntoView {
    view! {
        <div class="toast-container">
            {move || toasts.get().iter().map(|toast| {
                let class = match toast.level {
                    ToastLevel::Info => "toast toast--info",
                    ToastLevel::Success => "toast toast--success",
                    ToastLevel::Warning => "toast toast--warning",
                    ToastLevel::Error => "toast toast--error",
                };
                view! {
                    <div class=class>
                        <span class="toast-message">{toast.message.clone()}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
