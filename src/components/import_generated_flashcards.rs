use leptos::*;

use crate::components::page_title::*;

#[component]
pub fn ImportGeneratedFlashcards(import_string: String) -> impl IntoView {
    view! {
        <PageTitleComponent text="Import your flashcards!"/>

        <div class="text-center m-auto p-2 max-w-7xl">
            <button
                class="btn btn-accent w-full"
                on:click=move |_| {
                    if let Some(clipboard) = window().navigator().clipboard() {
                        //TODO: Notifications
                        let _no = clipboard.write_text(&import_string);
                    } else {
                        //TODO: Notifications
                    }
                }
            >
                "Copy to Clipboard"
            </button>
        </div>
    }
}
