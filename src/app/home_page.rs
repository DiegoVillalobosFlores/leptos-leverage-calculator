use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Interesante"</h1>
        <a href="/calculator">calculator</a>
    }
}