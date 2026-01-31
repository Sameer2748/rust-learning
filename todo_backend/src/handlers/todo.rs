
use axum::extract::{Json,State, Path, Extension};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;


#[derive(Debug, Deserialize, Serialize)]
pub struct TodoData {
    pub title: String,
    pub description: String,
    pub tag: String
}

pub async fn getalltodo()-> String{
    String::from("hey from todo 1, hey from todo 2")
}

// so here this userdata is like a type that tell code that this json will be give data like this userdata  
pub async  fn createTodo(State(state): State<AppState>, Json(todo): Json<TodoData>, Extension(user_id): Extension<i32>)-> String{
    println!("todo is : {:?},  userid is :{}", todo, user_id );
    // do the db creation of the todo 
    format!("Created todo: {}", todo.title)
} 

pub async  fn updateTodo(Path(id): Path<u32>){
    println!("{}", id);
}
pub async  fn getTodo(Path(id): Path<u32>){
    println!("{}", id);
}
pub async  fn deleteTodo(Path(id): Path<u32>){
    println!("{}", id);
}