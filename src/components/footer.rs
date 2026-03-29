use leptos::*;
use leptos_router::*;
use crate::model::Todo;
use crate::server::clear_completed;

#[component]
pub fn Footer(
    todos: ReadSignal<Vec<Todo>>,
    on_change: Callback<()>,
) -> impl IntoView {
    let location = use_location();

    let active_count = move || todos.get().iter().filter(|t| !t.completed).count();
    let completed_count = move || todos.get().iter().filter(|t| t.completed).count();
    let has_todos = move || !todos.get().is_empty();

    let current_filter = move || location.pathname.get();

    let handle_clear_completed = {
        let on_change = on_change.clone();
        move |_| {
            let on_change = on_change.clone();
            spawn_local(async move {
                if clear_completed().await.is_ok() {
                    leptos::Callable::call(&on_change, ());
                }
            });
        }
    };

    view! {
        <Show when=has_todos>
            <footer class="footer">
                <span class="todo-count">
                    <strong>{active_count}</strong>
                    {move || if active_count() == 1 { " item left" } else { " items left" }}
                </span>
                <ul class="filters">
                    <li>
                        <a
                            href="/"
                            class=move || if current_filter() == "/" { "selected" } else { "" }
                        >
                            "All"
                        </a>
                    </li>
                    <li>
                        <a
                            href="/active"
                            class=move || if current_filter() == "/active" { "selected" } else { "" }
                        >
                            "Active"
                        </a>
                    </li>
                    <li>
                        <a
                            href="/completed"
                            class=move || if current_filter() == "/completed" { "selected" } else { "" }
                        >
                            "Completed"
                        </a>
                    </li>
                </ul>
                <Show when=move || { completed_count() > 0 }>
                    <button
                        class="clear-completed"
                        on:click=handle_clear_completed.clone()
                    >
                        "Clear completed"
                    </button>
                </Show>
            </footer>
        </Show>
    }
}
