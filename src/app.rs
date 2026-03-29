use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::model::Todo;
use crate::server::get_todos;
use crate::components::header::Header;

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

    // Load todos on mount
    let refresh_clone = refresh_todos.clone();
    create_effect(move |_| {
        refresh_clone();
    });

    let on_add = Callback::new(move |_: ()| {
        refresh_todos();
    });

    view! {
        <section class="todoapp">
            <Header on_add=on_add/>
            <section class="main">
                <ul class="todo-list">
                    {move || todos.get().iter().map(|todo| {
                        view! {
                            <li>
                                <div class="view">
                                    <label>{todo.title.clone()}</label>
                                </div>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </section>
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
