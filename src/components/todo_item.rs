use leptos::*;
use crate::model::Todo;
use crate::server::{toggle_todo, delete_todo};

#[component]
pub fn TodoItem(
    todo: Todo,
    on_change: Callback<()>,
) -> impl IntoView {
    let todo_id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);
    let title = todo.title.clone();

    let handle_toggle = {
        let on_change = on_change.clone();
        move |_| {
            let on_change = on_change.clone();
            spawn_local(async move {
                if toggle_todo(todo_id).await.is_ok() {
                    set_completed.update(|c| *c = !*c);
                    leptos::Callable::call(&on_change, ());
                }
            });
        }
    };

    let handle_delete = {
        let on_change = on_change.clone();
        move |_| {
            let on_change = on_change.clone();
            spawn_local(async move {
                if delete_todo(todo_id).await.is_ok() {
                    leptos::Callable::call(&on_change, ());
                }
            });
        }
    };

    view! {
        <li class=move || if completed.get() { "completed" } else { "" }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=handle_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
        </li>
    }
}
