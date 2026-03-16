use axum::{
    extract::{Extension, Json, Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, db::todo_fns::{create_todo, delete_todo, get_todo, get_todo_by_id, update_todo}};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoData {
    pub title: String,
    pub description: String,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub tag: String,
    pub user_id: i32,
}

pub async fn getalltodo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>
) -> impl IntoResponse {
   let todos=  get_todo(&state.pool, user_id).await;
   match todos {
    Ok(todos) => (axum::http::StatusCode::OK, Json(todos)).into_response(),
    Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
   }
}

// so here this userdata is like a type that tell code that this json will be give data like this userdata
// founded a issue the json always need to be in last argument becuase it extract the req body
pub async fn create_Todo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>,
    Json(todo): Json<TodoData>,
) -> impl IntoResponse {
    println!("todo is : {:?},  userid is :{}", todo, user_id);
    // do the db creation of the todo
    match create_todo(&state.pool, &todo, user_id).await {
        Ok(id) => (axum::http::StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


pub async fn getTodo(State(state): State<AppState>,Path(id): Path<i32>, Extension(user_id): Extension<i32>)-> impl IntoResponse {
    let todo = get_todo_by_id(&state.pool, user_id, id).await;
    match todo {
        Ok(todo) => (axum::http::StatusCode::OK, Json(todo)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


pub async fn updateTodo(State(state): State<AppState>,Path(id): Path<i32>, Extension(user_id): Extension<i32>, Json(todo): Json<TodoData>)-> impl IntoResponse{
    let todo = update_todo(&state.pool, user_id, id, &todo).await;
    match todo {
        Ok(todo) => (axum::http::StatusCode::OK, Json(todo)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
       }
}

pub async fn deleteTodo(State(state): State<AppState>, Path(id): Path<i32>, Extension(user_id): Extension<i32>)-> impl IntoResponse {
    let todo = delete_todo(&state.pool, user_id, id).await;
    match todo {
        Ok(todo) => (axum::http::StatusCode::OK, Json(todo)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
       }
}
