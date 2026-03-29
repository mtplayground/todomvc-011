use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::model::Todo;
use crate::server::{get_todos, toggle_all};
use crate::components::header::Header;
use crate::components::todo_list::TodoList;
use crate::components::footer::Footer;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc.css"/>
        <Router>
            <Routes>
                <Route path="/" view=TodoApp/>
                <Route path="/active" view=TodoApp/>
                <Route path="/completed" view=TodoApp/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());

    let refresh_todos = move || {
        spawn_local(async move {
            if let Ok(fetched) = get_todos().await {
                set_todos.set(fetched);
            }
        });
    };

    let refresh_clone = refresh_todos.clone();
    create_effect(move |_| {
        refresh_clone();
    });

    let on_change: Callback<()> = Callback::new(move |_: ()| {
        refresh_todos();
    });

    let on_add: Callback<()> = {
        let on_change = on_change.clone();
        Callback::new(move |_: ()| {
            leptos::Callable::call(&on_change, ());
        })
    };

    let all_completed = move || {
        let t = todos.get();
        !t.is_empty() && t.iter().all(|t| t.completed)
    };

    let has_todos = move || !todos.get().is_empty();

    let handle_toggle_all = {
        let on_change = on_change.clone();
        move |_| {
            let completed = !all_completed();
            let on_change = on_change.clone();
            spawn_local(async move {
                if toggle_all(completed).await.is_ok() {
                    leptos::Callable::call(&on_change, ());
                }
            });
        }
    };

    view! {
        <section class="todoapp">
            <Header on_add=on_add/>
            <section class="main">
                <Show when=has_todos>
                    <input
                        id="toggle-all"
                        class="toggle-all"
                        type="checkbox"
                        prop:checked=all_completed
                        on:change=handle_toggle_all
                    />
                    <label for="toggle-all">"Mark all as complete"</label>
                </Show>
                <TodoList todos=todos on_change=on_change.clone()/>
            </section>
            <Footer todos=todos on_change=on_change/>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
        </footer>
    }
}

pub fn shell(_options: leptos::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
