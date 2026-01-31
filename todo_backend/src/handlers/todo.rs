
use axum::extract::{Json, Path};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct TodoData {
    pub id : i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub tag: String
}

pub async fn getalltodo()-> String{
    String::from("hey from todo 1, hey from todo 2")
}

// so here this userdata is like a type that tell code that this json will be give data like this userdata  
pub async  fn createTodo(Json(todo): Json<TodoData>)-> String{
    println!("todo is : {:?}", todo );
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