use leptos::*;
use leptos::ev::Event;

#[component]
pub fn Calculator(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (name, set_name) = create_signal(cx, "Name".to_string());

    let on_click = move |_| set_count.update(|count| *count += 1);
    let on_input = move |e: Event | set_name(event_target_value(&e));

    view! { cx,
        <h1>"Leverage calculator"</h1>
        <button on:click=on_click>{name}":" {count}</button>
        <br/>
        <input
            placeholder="lele"
            type="text"
            on:input=on_input
            prop:value=name
        />
        <br/>
        <a href="/">Home</a>
    }
}