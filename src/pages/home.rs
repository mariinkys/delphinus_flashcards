use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="my-16 xl:my-80 text-center w-full p-3">
            <h1 class="text-5xl font-bold">"Welcome to Delphinus Flashcards"</h1>
            <p class="py-6">"This software will allow you to generate flashcards for both Chinese and Japanese with ease,
            the flashcards generated can later be imported to sites like Quizlet or Vaia(Studysmarter)."</p>
            <a href="/generator" class="btn btn-primary">"Generate your Flashcards"</a>
        </div>
    }
}
