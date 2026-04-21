use leptos::prelude::*;

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let set_theme = move |theme: &'static str| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute(
                        "data-theme",
                        theme,
                    );
                }

                if let Ok(Some(storage)) =
                    window.local_storage()
                {
                    let _ =
                        storage.set_item(
                            "theme",
                            theme,
                        );
                }
            }
        }
    };

    view! {
        <button
            on:click=move |_| {
                set_theme("night")
            }
        >
            "Night"
        </button>
    }
}