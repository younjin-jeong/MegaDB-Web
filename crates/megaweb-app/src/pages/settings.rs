use leptos::prelude::*;

/// Settings page.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let (theme, set_theme) = signal("dark".to_string());
    let (font_size, set_font_size) = signal(14u32);

    view! {
        <div class="settings-page">
            <h2>"Settings"</h2>

            <div class="settings-section">
                <h3>"Appearance"</h3>
                <div class="setting-item">
                    <label>"Theme"</label>
                    <select
                        prop:value=move || theme.get()
                        on:change=move |ev| set_theme.set(event_target_value(&ev))
                    >
                        <option value="dark">"Dark"</option>
                        <option value="light">"Light"</option>
                    </select>
                </div>
                <div class="setting-item">
                    <label>"Editor Font Size"</label>
                    <input
                        type="number"
                        min="10"
                        max="24"
                        prop:value=move || font_size.get().to_string()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                set_font_size.set(v);
                            }
                        }
                    />
                </div>
            </div>

            <div class="settings-section">
                <h3>"Query"</h3>
                <div class="setting-item">
                    <label>"Default Row Limit"</label>
                    <input type="number" value="1000" min="100" max="100000" />
                </div>
                <div class="setting-item">
                    <label>"Auto-complete"</label>
                    <input type="checkbox" checked />
                </div>
            </div>

            <div class="settings-section">
                <h3>"About"</h3>
                <p>"MegaDB Web Client v0.1.0"</p>
                <p>"Built with Leptos + Rust WASM"</p>
                <p>
                    <a href="https://github.com/younjin-jeong/MegaDB-Web" target="_blank">
                        "GitHub Repository"
                    </a>
                </p>
            </div>
        </div>
    }
}
