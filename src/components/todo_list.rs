use leptos::*;
use leptos_router::use_location;
use crate::model::Todo;
use crate::components::todo_item::TodoItem;

#[component]
pub fn TodoList(
    todos: ReadSignal<Vec<Todo>>,
    on_change: Callback<()>,
) -> impl IntoView {
    let location = use_location();

    let filtered_todos = move || {
        let path = location.pathname.get();
        let all = todos.get();
        match path.as_str() {
            "/active" => all.into_iter().filter(|t| !t.completed).collect::<Vec<_>>(),
            "/completed" => all.into_iter().filter(|t| t.completed).collect::<Vec<_>>(),
            _ => all,
        }
    };

    view! {
        <ul class="todo-list">
            <For
                each=filtered_todos
                key=|todo| todo.id
                children={
                    let on_change = on_change.clone();
                    move |todo| {
                        let on_change = on_change.clone();
                        view! {
                            <TodoItem todo=todo on_change=on_change/>
                        }
                    }
                }
            />
        </ul>
    }
}
