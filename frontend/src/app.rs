use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{AnimalType, ChatSession, Language};
use crate::i18n::get_translations;

use crate::components::chat_area::ChatArea;
use crate::components::sidebar::Sidebar;
use crate::components::update_banner::UpdateBanner;

const STORAGE_KEY: &str = "ai_animal_chats_v1";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppState {
    pub chats: Vec<ChatSession>,
    pub active_chat_id: Option<String>,
    #[serde(default)]
    pub language: Language,
}

#[component]
pub fn App() -> impl IntoView {
    let initial_state: AppState = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

    let chats: RwSignal<Vec<ChatSession>> = RwSignal::new(initial_state.chats);
    let active_chat_id: RwSignal<Option<String>> = RwSignal::new(initial_state.active_chat_id);
    let language: RwSignal<Language> = RwSignal::new(initial_state.language);
    let sidebar_open: RwSignal<bool> = RwSignal::new(false);
    let is_thinking: RwSignal<bool> = RwSignal::new(false);

    let i18n = Memo::new(move |_| get_translations(language.get()));

    let animal = Memo::new(move |_| {
        active_chat_id
            .get()
            .and_then(|id| chats.get().into_iter().find(|c| c.id == id))
            .map(|c| c.animal)
            .unwrap_or_default()
    });

    provide_context(chats);
    provide_context(active_chat_id);
    provide_context(language);
    provide_context(sidebar_open);
    provide_context(is_thinking);
    provide_context(animal);
    provide_context(i18n);

    Effect::new(move || {
        let state = AppState {
            chats: chats.get(),
            active_chat_id: active_chat_id.get(),
            language: language.get(),
        };
        let _ = LocalStorage::set(STORAGE_KEY, state);
    });

    Effect::new(move || {
        let theme = match animal.get() {
            AnimalType::Cat => "cat",
            AnimalType::Octopus => "octopus",
            AnimalType::Elephant => "elephant",
            AnimalType::Chicken => "chicken",
        };
        if let Some(body) = document().body() {
            let _ = body.set_attribute("data-theme", theme);
        }
    });

    view! {
        <div class="layout" class:sidebar-open=move || sidebar_open.get()>
            <Sidebar />
            <ChatArea />
            <UpdateBanner />
        </div>
    }
}
