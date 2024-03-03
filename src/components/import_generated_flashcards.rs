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
                    //Copy import_string to clipboard
                }
            >
                "Copy to Clipboard"
            </button>
        </div>

        <p class="mt-3">"DEBUG: "</p>
        <p>{import_string}</p>
    }
}
