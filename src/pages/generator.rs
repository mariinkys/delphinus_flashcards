use leptos::{ev::SubmitEvent, *};
use leptos_router::use_navigate;

use crate::components::modify_generated_flashcards::ModifyGeneratedFlashcards;
use crate::components::{page_title::*, select_option::*};
use crate::utils::{remove_whitespace, search_dictionary, Flashcard};

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let (character_string, set_character_string) = create_signal("".to_string());
    let (language, set_language) = create_signal("Chinese".to_string());
    let (results, set_results) = create_signal(Vec::<Flashcard>::new());
    let (loading, set_loading) = create_signal(false);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        set_loading(true);
        let navigate = use_navigate();

        //Check data
        if character_string.get().len() > 0 {
            if language.get() == "Chinese" {
                spawn_local(async move {
                    let clean_input = remove_whitespace(&character_string.get_untracked());

                    match search_dictionary(clean_input, true).await {
                        Ok(found_results) => {
                            if found_results.is_empty() {
                                set_loading(false);
                                navigate("/noresults", Default::default());
                            } else {
                                set_loading(false);
                                set_results(found_results)
                            }
                        }
                        Err(_) => {
                            set_loading(false);
                            navigate("/noresults", Default::default());
                        }
                    }
                });
            } else if language.get() == "Japanese" {
                spawn_local(async move {
                    let clean_input = remove_whitespace(&character_string.get_untracked());

                    match search_dictionary(clean_input, false).await {
                        Ok(found_results) => {
                            if found_results.is_empty() {
                                set_loading(false);
                                navigate("/noresults", Default::default());
                            } else {
                                set_loading(false);
                                set_results(found_results)
                            }
                        }
                        Err(_) => {
                            set_loading(false);
                            navigate("/noresults", Default::default());
                        }
                    }
                });
            }
        } else {
            //TODO: Error handling (:
            set_loading(false);
            navigate("/noresults", Default::default());
        }
    };

    view! {
        <Show
            when=move || { results().is_empty() }
            fallback=move || view! { <ModifyGeneratedFlashcards flashcards=results.get()/> }
        >
            <PageTitleComponent text="Generate Flashcards!"/>

            <div class="max-w-4xl text-center m-auto p-2">
                <div class="flex justify-end my-3">
                    <div class="tooltip tooltip-left" data-tip="For Japanese characters, you can separate the characters with the Japanese '、' for Chinese characters, you can use the Chinese '，' for both of them, you can use the traditional ',' however, you cannot mix them within the same input.">
                        <button class="btn btn-xs">"i"</button>
                    </div>
                </div>

                <form class="w-full" on:submit=on_submit>
                    <textarea
                        class="textarea textarea-bordered w-full h-80" placeholder="Enter character string"
                        prop:value=move || character_string.get()
                        on:input=move |ev| { set_character_string(event_target_value(&ev)) }
                        disabled=loading
                    >
                    </textarea>
                    <br/>
                    <select
                        class="select select-bordered w-full mt-3"
                        on:change=move |ev| {
                            let new_language = event_target_value(&ev);
                            set_language(new_language);
                        }
                        disabled=loading
                    >
                        <SelectOption value=language is="Chinese"/>
                        <SelectOption value=language is="Japanese"/>
                    </select>
                    <br/>
                    <button class="btn btn-primary mt-3 w-full" type="submit" disabled=loading>"Generate"</button>
                </form>
            </div>
        </Show>

    }
}
