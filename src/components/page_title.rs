use leptos::prelude::*;

#[component]
pub fn PageTitleComponent(text: &'static str) -> impl IntoView {
    view! {
        <h1 class="text-3xl text-center font-bold p-3">{text}</h1>
    }
}
