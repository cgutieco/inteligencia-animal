use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Banner that shows when a new Service Worker version is available.
#[component]
pub fn UpdateBanner() -> impl IntoView {
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(window) = document().default_view() {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                visible.set(true);
            }) as Box<dyn Fn(web_sys::Event)>);

            let _ = window.add_event_listener_with_callback(
                "swUpdateAvailable",
                closure.as_ref().unchecked_ref(),
            );

            closure.forget();
        }
    });

    let on_update_click = move |_| {
        if let Some(window) = document().default_view() {
            if let Ok(event) = web_sys::CustomEvent::new("swSkipWaiting") {
                let _ = window.dispatch_event(&event);
            }
        }
    };

    view! {
        <Show when=move || visible.get()>
            <div class="sw-update-banner" role="alert">
                <span>"Nueva versión disponible"</span>
                <button
                    type="button"
                    class="sw-update-btn"
                    on:click=on_update_click
                >
                    "Actualizar"
                </button>
            </div>
        </Show>
    }
}





