use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod home_page;
mod not_found;

mod calculator;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos.css"/>

        // sets the document title
        <Title text="Leverage Calculator"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=home_page::HomePage/>
                    <Route path="/*any" view=not_found::NotFound/>
                    <Route path="/calculator" view=calculator::Calculator/>
                </Routes>
            </main>
        </Router>
    }
}

