use crate::components::config_panel::ConfigPanel;
use crate::components::context_menu::ContextMenu;
use crate::i18n::Translations;
use leptos::prelude::*;
use shared::{AnimalType, ChatSession, IntelligenceLevel, Language};

/// Sidebar with chat button, chat history list, and config panel.
#[component]
pub fn Sidebar() -> impl IntoView {
    let sidebar_open = use_context::<RwSignal<bool>>().expect("sidebar_open");
    let chats = use_context::<RwSignal<Vec<ChatSession>>>().expect("chats");
    let active_chat_id = use_context::<RwSignal<Option<String>>>().expect("active_chat_id");
    let language = use_context::<RwSignal<Language>>().expect("language");
    let i18n = use_context::<Memo<Translations>>().expect("i18n");

    let menu_open_for = RwSignal::new(Option::<String>::None);

    let on_new_chat = move |_| {
        let current_lang = language.get();
        let mut new_chat =
            ChatSession::new(AnimalType::Cat, IntelligenceLevel::Medium, current_lang);
        new_chat.title = i18n.get().new_conversation.to_string();
        let id = new_chat.id.clone();

        chats.update(|v| v.insert(0, new_chat));
        active_chat_id.set(Some(id));
        sidebar_open.set(false);
    };

    let delete_chat = move |id: String| {
        chats.update(|v| v.retain(|c| c.id != id));
        if active_chat_id.get() == Some(id) {
            active_chat_id.set(chats.get().first().map(|c| c.id.clone()));
        }
    };

    let rename_chat = move |id: String| {
        let prompt_text = i18n.get().rename_dialog_title;
        if let Some(Some(new_name)) = window().prompt_with_message(prompt_text).ok() {
            if !new_name.trim().is_empty() {
                chats.update(|v| {
                    if let Some(chat) = v.iter_mut().find(|c| c.id == id) {
                        chat.title = new_name.trim().to_string();
                    }
                });
            }
        }
    };

    view! {
        <aside class="sidebar" class:open=move || sidebar_open.get()>
            <div class="sidebar-header">
                <div class="sidebar-brand">
                    {move || i18n.get().app_title}
                </div>
                <button class="new-chat-btn" on:click=on_new_chat>
                    <span class="material-symbols-outlined">{"edit"}</span>
                    {move || i18n.get().new_chat}
                </button>
            </div>

            <div class="sidebar-section-title">{move || i18n.get().chats_title}</div>

            <div class="chat-list">
                <For
                    each=move || chats.get()
                    key=|chat| chat.id.clone()
                    children=move |chat| {
                        let id = chat.id.clone();

                        let id_for_click = id.clone();
                        let id_for_signal = id.clone();
                        let id_for_menu_click = id.clone();
                        let id_for_rename = id.clone();
                        let id_for_delete = id.clone();

                        let is_active = move || active_chat_id.get() == Some(id.clone());
                        let is_menu_open = Signal::derive(move || menu_open_for.get() == Some(id_for_signal.clone()));

                        view! {
                            <div
                                class="chat-item"
                                class:active=is_active
                                on:click=move |_| {
                                    active_chat_id.set(Some(id_for_click.clone()));
                                    sidebar_open.set(false);
                                }
                            >
                                <span class="chat-item-title">{chat.title}</span>
                                <button
                                    class="chat-item-menu-btn"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        let current_id = id_for_menu_click.clone();
                                        menu_open_for.update(|v| {
                                            *v = if *v == Some(current_id.clone()) { None } else { Some(current_id) };
                                        });
                                    }
                                    aria-label="Opciones del chat"
                                >
                                    <span class="material-symbols-outlined">{"more_vert"}</span>
                                </button>
                                <ContextMenu
                                    visible=is_menu_open
                                    on_close=Callback::new(move |_| menu_open_for.set(None))
                                    on_rename=Callback::new(move |_| rename_chat(id_for_rename.clone()))
                                    on_delete=Callback::new(move |_| delete_chat(id_for_delete.clone()))
                                />
                            </div>
                        }
                    }
                />
            </div>

            <ConfigPanel />
        </aside>

        <div
            class="sidebar-overlay"
            class:open=move || sidebar_open.get()
            on:click=move |_| sidebar_open.set(false)
        ></div>
    }
}
