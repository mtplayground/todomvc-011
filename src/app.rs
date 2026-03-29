use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc.css"/>
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hello, TodoMVC!"</h1>
    }
}

pub fn shell(options: leptos::LeptosOptions) -> impl IntoView {
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
