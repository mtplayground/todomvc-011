use leptos::*;
use crate::server::add_todo;

#[component]
pub fn Header(on_add: Callback<()>) -> impl IntoView {
    let (new_title, set_new_title) = create_signal(String::new());

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            let title = new_title.get();
            let title = title.trim().to_string();
            if !title.is_empty() {
                let on_add = on_add.clone();
                spawn_local(async move {
                    if add_todo(title).await.is_ok() {
                        leptos::Callable::call(&on_add, ());
                    }
                });
                set_new_title.set(String::new());
            }
        }
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus
                prop:value=new_title
                on:input=move |ev| set_new_title.set(event_target_value(&ev))
                on:keydown=handle_keydown
            />
        </header>
    }
}
