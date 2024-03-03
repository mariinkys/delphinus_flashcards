use leptos::*;

use crate::components::page_title::*;

/// Renders the home page of your application.
#[component]
pub fn NoResultsPage() -> impl IntoView {
    view! {
        <PageTitleComponent text="No Results"/>
    }
}
