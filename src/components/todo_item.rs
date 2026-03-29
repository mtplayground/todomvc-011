use leptos::*;
use crate::model::Todo;
use crate::server::{toggle_todo, delete_todo, update_todo_title};

#[component]
pub fn TodoItem(
    todo: Todo,
    on_change: Callback<()>,
) -> impl IntoView {
    let todo_id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);
    let (editing, set_editing) = create_signal(false);
    let (edit_title, set_edit_title) = create_signal(todo.title.clone());
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

    let handle_dblclick = move |_| {
        set_editing.set(true);
    };

    let commit_edit = {
        let on_change = on_change.clone();
        move || {
            let new_title = edit_title.get();
            let new_title = new_title.trim().to_string();
            let on_change = on_change.clone();
            if new_title.is_empty() {
                // Delete todo if title is empty
                spawn_local(async move {
                    if delete_todo(todo_id).await.is_ok() {
                        leptos::Callable::call(&on_change, ());
                    }
                });
            } else {
                spawn_local(async move {
                    if update_todo_title(todo_id, new_title).await.is_ok() {
                        leptos::Callable::call(&on_change, ());
                    }
                });
            }
            set_editing.set(false);
        }
    };

    let commit_clone = commit_edit.clone();
    let handle_blur = move |_| {
        commit_clone();
    };

    let commit_clone2 = commit_edit.clone();
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => commit_clone2(),
            "Escape" => {
                set_edit_title.set(title.clone());
                set_editing.set(false);
            }
            _ => {}
        }
    };

    view! {
        <li class=move || {
            let mut classes = Vec::new();
            if completed.get() { classes.push("completed"); }
            if editing.get() { classes.push("editing"); }
            classes.join(" ")
        }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=handle_toggle
                />
                <label on:dblclick=handle_dblclick>{move || edit_title.get()}</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
            <input
                class="edit"
                prop:value=edit_title
                on:input=move |ev| set_edit_title.set(event_target_value(&ev))
                on:blur=handle_blur
                on:keydown=handle_keydown
            />
        </li>
    }
}
