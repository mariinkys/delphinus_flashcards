use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::navbar::NavbarComponent,
    pages::{faq::*, generator::*, home::*, no_results::*, not_found::*},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/delphinus.css"/>

        // sets the document title
        <Title text="Delphinus Flashcards"/>

        <NavbarComponent/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/generator" view=GeneratorPage/>
                    <Route path="/noresults" view=NoResultsPage/>
                    <Route path="/faq" view=FaqPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
