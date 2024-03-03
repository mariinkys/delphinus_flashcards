use leptos::*;

use crate::{
    components::{
        import_generated_flashcards::ImportGeneratedFlashcards, page_title::PageTitleComponent,
    },
    pages::no_results::NoResultsPage,
    utils::{create_import_string, Flashcard},
};

#[component]
pub fn ModifyGeneratedFlashcards(flashcards: Vec<Flashcard>) -> impl IntoView {
    let (data, set_data) = create_signal(flashcards);
    let (import_string, set_import_string) = create_signal(String::from(""));

    view! {
        <Show
            when=move || { import_string.get().len() == 0 }
            fallback=move || view! { <ImportGeneratedFlashcards import_string=import_string.get()/> }
        >

            <PageTitleComponent text="Modify Flashcards!"/>

            <Show
                when=move || { !data.get().is_empty() }
                fallback=move || view! { <NoResultsPage/> }
            >
                <div class="text-center m-auto p-2 max-w-7xl">
                    <For
                        each=data
                        key=|state| (state.id.clone(), state.id)
                        let:child
                    >
                        <div class="flex flex-col sm:flex-row gap-2 p-2 border rounded-lg my-2 shadow-lg">
                            <label class="input input-bordered flex items-center gap-2 font-bold">
                                "Front"
                                <input type="text" class="grow font-normal" name="front" id="front" value=child.front on:input=move |ev| {
                                    child.front.set(event_target_value(&ev));
                                }/>
                            </label>
                            <label class="input input-bordered flex items-center gap-2 font-bold flex-grow">
                                "Back"
                                <input type="text" class="grow font-normal" name="back" id="back" value=child.back on:input=move |ev| {
                                    child.back.set(event_target_value(&ev));
                                }/>
                            </label>
                            <button class="btn btn-error" on:click=move |_| {
                                set_data.update(|data| data.retain(|flashcard| flashcard.id != child.id));
                            }>"Delete"</button>
                        </div>
                    </For>
                    <button class="btn btn-primary mt-1 w-full" on:click=move |_| {
                        //Process the data
                        set_import_string(create_import_string(data.get()))
                    }>"Generate"</button>
                </div>
            </Show>
        </Show>
    }
}
