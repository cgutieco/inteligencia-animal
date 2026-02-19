use leptos::prelude::*;

/// A single chat message bubble.
#[component]
pub fn ChatBubble(
    /// "user" or "assistant"
    #[prop(into)]
    role: String,
    /// The message content
    #[prop(into)]
    content: String,
) -> impl IntoView {
    let role_class = role.clone();

    // Convert markdown to HTML
    let html_content = markdown::to_html(&content);

    view! {
        <div class={format!("bubble-row {}", role_class)}>
            <div class={format!("bubble {}", role)} inner_html=html_content>
            </div>
        </div>
    }
}

/// Animated thinking indicator (three bouncing dots).
#[component]
pub fn ThinkingBubble() -> impl IntoView {
    view! {
        <div class="bubble-row assistant">
            <div class="bubble assistant">
                <div class="thinking-dots">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>
        </div>
    }
}
