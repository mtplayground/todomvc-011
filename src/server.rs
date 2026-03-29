use crate::model::Todo;
use leptos::*;

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use crate::model::db;
    let pool = db::get_db().await;
    db::get_todos(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use crate::model::db;
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::ServerError("Title cannot be empty".to_string()));
    }
    let pool = db::get_db().await;
    db::add_todo(&pool, title)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(id: i64) -> Result<(), ServerFnError> {
    use crate::model::db;
    let pool = db::get_db().await;
    db::toggle_todo(&pool, id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(UpdateTodoTitle, "/api")]
pub async fn update_todo_title(id: i64, title: String) -> Result<(), ServerFnError> {
    use crate::model::db;
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::ServerError("Title cannot be empty".to_string()));
    }
    let pool = db::get_db().await;
    db::update_todo_title(&pool, id, title)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use crate::model::db;
    let pool = db::get_db().await;
    db::delete_todo(&pool, id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(ToggleAll, "/api")]
pub async fn toggle_all(completed: bool) -> Result<(), ServerFnError> {
    use crate::model::db;
    let pool = db::get_db().await;
    db::toggle_all_todos(&pool, completed)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(ClearCompleted, "/api")]
pub async fn clear_completed() -> Result<(), ServerFnError> {
    use crate::model::db;
    let pool = db::get_db().await;
    db::clear_completed_todos(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
