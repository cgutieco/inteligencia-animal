use crate::components::animal_card::AnimalCard;
use crate::components::chat_bubble::{ChatBubble, ThinkingBubble};
use leptos::task::spawn_local;
use leptos::prelude::*;
use shared::{AnimalType, ChatSession, ChatMessage, Role, ChatRequest, ChatResponse, Language};
use gloo_net::http::Request;
use crate::i18n::Translations;

fn add_message(chats: RwSignal<Vec<ChatSession>>, chat_id: &str, msg: ChatMessage) {
    chats.update(|v| {
        if let Some(chat) = v.iter_mut().find(|c| c.id == chat_id) {
            chat.messages.push(msg);
        }
    });
}

/// Main chat area with messages, empty state, and input bar.
#[component]
pub fn ChatArea() -> impl IntoView {
    let chats = use_context::<RwSignal<Vec<ChatSession>>>().expect("chats context");
    let active_chat_id = use_context::<RwSignal<Option<String>>>().expect("active_chat_id context");
    let sidebar_open = use_context::<RwSignal<bool>>().expect("sidebar_open context");
    let is_thinking = use_context::<RwSignal<bool>>().expect("is_thinking");
    let _language = use_context::<RwSignal<Language>>().expect("language");
    let i18n = use_context::<Memo<Translations>>().expect("i18n");
    
    let animal = use_context::<Memo<AnimalType>>().expect("AnimalType");

    let input_value = RwSignal::new(String::new());

    let active_chat = Memo::new(move |_| {
        let id = active_chat_id.get();
        chats.get().into_iter().find(|c| Some(c.id.clone()) == id)
    });

    let send_message = move || {
        let text = input_value.get();
        if text.trim().is_empty() || is_thinking.get() {
            return;
        }

        let current_id = match active_chat_id.get() {
            Some(id) => id,
            None => return,
        };

        let user_msg = ChatMessage {
            role: Role::User,
            content: text.clone(),
        };

        // 1. Add User Message
        add_message(chats, &current_id, user_msg);

        input_value.set(String::new());
        is_thinking.set(true);

        spawn_local(async move {
            let chat_opt = chats.get().into_iter().find(|c| c.id == current_id);
            if let Some(chat) = chat_opt {
                let req = ChatRequest {
                    message: text,
                    animal: chat.animal,
                    intelligence: chat.intelligence,
                    history: chat.messages[..chat.messages.len()-1].to_vec(),
                };

                let response = Request::post("/api/chat")
                    .json(&req)
                    .expect("Failed to serialize request")
                    .send()
                    .await;

                match response {
                    Ok(res) if res.ok() => {
                        if let Ok(data) = res.json::<ChatResponse>().await {
                            let assistant_msg = ChatMessage {
                                role: Role::Assistant,
                                content: data.response,
                            };
                            // 2. Add Assistant Message
                            add_message(chats, &current_id, assistant_msg);
                        }
                    }
                    _ => {
                        let error_msg = ChatMessage {
                            role: Role::Assistant,
                            content: i18n.get().error_message.to_string(),
                        };
                        // 3. Add Error Message
                        add_message(chats, &current_id, error_msg);
                    }
                }
            }
            is_thinking.set(false);
        });
    };

    view! {
        <main class="chat-area">
            // Header bar (mobile only)
            <div class="header-bar">
                <button
                    class="hamburger-btn"
                    on:click=move |_| {
                        sidebar_open.update(|v| {
                            *v = !*v;
                        })
                    }
                    aria-label="Abrir menú"
                >
                    <span class="material-symbols-outlined">{"menu"}</span>
                </button>
                <h1>{move || i18n.get().app_title}</h1>
            </div>

            // Animal watermark
            <AnimalCard />

            // Messages area
            <div class="chat-messages" role="log" aria-live="polite">
                {move || {
                    if let Some(chat) = active_chat.get() {
                        if chat.messages.is_empty() {
                            view! {
                                <div class="empty-state">
                                    <div class="empty-state-title">
                                        {move || {
                                            match animal.get() {
                                                AnimalType::Cat => i18n.get().cat_sound,
                                                AnimalType::Octopus => i18n.get().octopus_sound,
                                                AnimalType::Elephant => i18n.get().elephant_sound,
                                                AnimalType::Chicken => i18n.get().chicken_sound,
                                            }
                                        }}
                                    </div>
                                    <div class="empty-state-subtitle">
                                        {move || i18n.get().empty_chat_subtitle}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            chat.messages.into_iter().map(|msg| {
                                let role_str = match msg.role {
                                    Role::User => "user",
                                    Role::Assistant => "assistant",
                                };
                                view! {
                                    <ChatBubble role=role_str.to_string() content=msg.content />
                                }
                            }).collect::<Vec<_>>().into_any()
                        }
                    } else {
                        view! {
                            <div class="empty-state">
                                <div class="empty-state-title">{move || i18n.get().select_chat}</div>
                                <div class="empty-state-subtitle">{move || i18n.get().select_chat_subtitle}</div>
                            </div>
                        }.into_any()
                    }
                }}

                // Thinking indicator
                <Show when=move || is_thinking.get()>
                    <ThinkingBubble />
                </Show>
            </div>

            // Input bar
            <div class="chat-input-container">
                <div class="chat-input-wrapper">
                    <input
                        type="text"
                        class="chat-input"
                        placeholder=move || i18n.get().send_placeholder
                        aria-label="Mensaje de chat"
                        prop:value=move || input_value.get()
                        on:input=move |ev| input_value.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                send_message();
                            }
                        }
                    />
                    <button 
                        class="send-btn" 
                        aria-label="Enviar mensaje"
                        on:click=move |_| send_message()
                        disabled=move || is_thinking.get()
                    >
                        <span class="material-symbols-outlined">{"send"}</span>
                    </button>
                </div>
            </div>
        </main>
    }
}
