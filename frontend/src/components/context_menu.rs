use leptos::prelude::*;
use crate::i18n::Translations;

/// A simple context menu for chat items.
#[component]
pub fn ContextMenu(
    #[prop(into)] visible: Signal<bool>,
    on_close: Callback<()>,
    on_rename: Callback<()>,
    on_delete: Callback<()>,
) -> impl IntoView {
    let i18n = use_context::<Memo<Translations>>().expect("i18n");
    view! {
        <Show when=move || visible.get()>
            <>
                <div class="context-menu-overlay" on:click=move |_| on_close.run(())></div>
                <div class="context-menu">
                    <button class="menu-item" on:click=move |_| {
                        on_rename.run(());
                        on_close.run(());
                    }>
                        <span class="material-symbols-outlined">{"edit"}</span>
                        {move || i18n.get().rename}
                    </button>
                    <button class="menu-item delete" on:click=move |_| {
                        on_delete.run(());
                        on_close.run(());
                    }>
                        <span class="material-symbols-outlined">{"delete"}</span>
                        {move || i18n.get().delete}
                    </button>
                </div>
            </>
        </Show>
    }
}
