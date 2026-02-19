use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[component]
pub fn CustomSelect(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] options: Signal<Vec<SelectOption>>,
    on_change: Callback<String>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    let toggle = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        open.update(|v| *v = !*v);
    };

    let select_option = move |val: String| {
        on_change.run(val);
        open.set(false);
    };

    let current_label = move || {
        let val = value.get();
        options.get().into_iter()
            .find(|opt| opt.value == val)
            .map(|opt| opt.label)
            .unwrap_or_default()
    };

    view! {
        <div class="custom-select-container" node_ref=wrapper_ref>
            <div 
                class="custom-select-trigger" 
                on:click=toggle
            >
                <span class="custom-select-label">{current_label}</span>
                <span class="material-symbols-outlined dropdown-icon">{"arrow_drop_down"}</span>
            </div>

            <Show when=move || open.get()>
                <div class="custom-select-overlay" on:click=move |_| open.set(false)></div>
                <div class="context-menu custom-select-menu">
                    <For
                        each=move || options.get()
                        key=|opt| opt.value.clone()
                        children=move |opt| {
                            let val = opt.value.clone();
                            let label = opt.label.clone();
                            let is_selected_val = val.clone();
                            let on_click_val = val.clone();
                            
                            let select_fn = select_option.clone();
                            let value_sig = value;
                            
                            view! {
                                <button 
                                    class="menu-item" 
                                    class:selected=move || value_sig.get() == is_selected_val
                                    on:click=move |_| select_fn(on_click_val.clone())
                                >
                                    <span class="material-symbols-outlined check-icon" style="opacity: 0">
                                        {"check"}
                                    </span>
                                    {label}
                                    <Show when=move || value_sig.get() == val>
                                        <span class="material-symbols-outlined check-icon" style="opacity: 1; margin-left: auto;">
                                            {"check"}
                                        </span>
                                    </Show>
                                </button>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
