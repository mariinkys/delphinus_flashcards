use leptos::prelude::*;

#[component]
pub fn NoResultsPage() -> impl IntoView {
    view! {
        <div class="hero min-h-[90vh]">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold py-6">"No results found"</h1>
                    <a href="/" class="btn btn-primary">"Return Home"</a>
                </div>
            </div>
        </div>
    }
}
