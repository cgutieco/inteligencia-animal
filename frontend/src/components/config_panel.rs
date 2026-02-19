use leptos::prelude::*;
use shared::{AnimalType, IntelligenceLevel, ChatSession, Language};
use crate::components::custom_select::{CustomSelect, SelectOption};

/// Configuration panel with Animal, Intelligence, and Language dropdowns.
#[component]
pub fn ConfigPanel() -> impl IntoView {
    let chats = use_context::<RwSignal<Vec<ChatSession>>>().expect("chats");
    let active_chat_id = use_context::<RwSignal<Option<String>>>().expect("active_chat_id");
    let language = use_context::<RwSignal<Language>>().expect("language");

    let animal = use_context::<Memo<AnimalType>>().expect("AnimalType");

    let current_settings = Memo::new(move |_| {
        active_chat_id.get().and_then(|id| {
            chats.get().into_iter().find(|c| c.id == id).map(|c| (c.animal, c.intelligence))
        })
    });

    let intelligence = move || current_settings.get().map(|(_, i)| i).unwrap_or(IntelligenceLevel::Medium);

    let update_animal = move |val: String| {
        if let Some(id) = active_chat_id.get() {
            let new_animal = match val.as_str() {
                "cat" => AnimalType::Cat,
                "octopus" => AnimalType::Octopus,
                "elephant" => AnimalType::Elephant,
                "chicken" => AnimalType::Chicken,
                _ => AnimalType::Cat,
            };
            chats.update(|v| {
                if let Some(chat) = v.iter_mut().find(|c| c.id == id) {
                    chat.animal = new_animal;
                }
            });
        }
    };

    let update_intelligence = move |val: String| {
        if let Some(id) = active_chat_id.get() {
            let new_iq = match val.as_str() {
                "high" => IntelligenceLevel::High,
                "medium" => IntelligenceLevel::Medium,
                "low" => IntelligenceLevel::Low,
                _ => IntelligenceLevel::Medium,
            };
            chats.update(|v| {
                if let Some(chat) = v.iter_mut().find(|c| c.id == id) {
                    chat.intelligence = new_iq;
                }
            });
        }
    };

    let update_language = move |val: String| {
        let new_lang = match val.as_str() {
            "es" => Language::Es,
            "en" => Language::En,
            _ => Language::Es,
        };
        language.set(new_lang);
    };

    let animal_options = Memo::new(move |_| vec![
        SelectOption { value: "cat".to_string(), label: AnimalType::Cat.label(language.get()).to_string() },
        SelectOption { value: "octopus".to_string(), label: AnimalType::Octopus.label(language.get()).to_string() },
        SelectOption { value: "elephant".to_string(), label: AnimalType::Elephant.label(language.get()).to_string() },
        SelectOption { value: "chicken".to_string(), label: AnimalType::Chicken.label(language.get()).to_string() },
    ]);

    let intelligence_options = Memo::new(move |_| vec![
        SelectOption { value: "high".to_string(), label: IntelligenceLevel::High.label(language.get()).to_string() },
        SelectOption { value: "medium".to_string(), label: IntelligenceLevel::Medium.label(language.get()).to_string() },
        SelectOption { value: "low".to_string(), label: IntelligenceLevel::Low.label(language.get()).to_string() },
    ]);

    let language_options = vec![
        SelectOption { value: "es".to_string(), label: "Español".to_string() },
        SelectOption { value: "en".to_string(), label: "English".to_string() },
    ];

    view! {
        <div class="config-panel">
            <div class="config-row">
                <span class="material-symbols-outlined">{"pets"}</span>
                <CustomSelect
                    value=Signal::derive(move || match animal.get() {
                        AnimalType::Cat => "cat",
                        AnimalType::Octopus => "octopus",
                        AnimalType::Elephant => "elephant",
                        AnimalType::Chicken => "chicken",
                    }.to_string())
                    options=Signal::derive(move || animal_options.get())
                    on_change=Callback::new(update_animal)
                />
            </div>

            <div class="config-row">
                <span class="material-symbols-outlined">{"psychology"}</span>
                <CustomSelect
                    value=Signal::derive(move || match intelligence() {
                        IntelligenceLevel::High => "high",
                        IntelligenceLevel::Medium => "medium",
                        IntelligenceLevel::Low => "low",
                    }.to_string())
                    options=Signal::derive(move || intelligence_options.get())
                    on_change=Callback::new(update_intelligence)
                />
            </div>

            <div class="config-row">
                <span class="material-symbols-outlined">{"language"}</span>
                <CustomSelect
                    value=Signal::derive(move || match language.get() {
                        Language::Es => "es",
                        Language::En => "en",
                    }.to_string())
                    options=Signal::derive(move || language_options.clone())
                    on_change=Callback::new(update_language)
                />
            </div>
        </div>
    }
}
