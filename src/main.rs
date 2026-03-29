#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use todomvc::app::App;
    use tower_http::services::ServeDir;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    let site_root = leptos_options.site_root.clone();
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
