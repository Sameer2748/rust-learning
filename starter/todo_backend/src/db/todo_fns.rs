use sqlx::PgPool;

use crate::handlers::todo::{Todo, TodoData};

pub async fn create_todo(pool: &PgPool, todo: &TodoData, user_id: i32) -> Result<i32, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, description, tag, user_id) VALUES ($1, $2 , $3 , $4) RETURNING *",
    )
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(&todo.tag)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(todo.id)
}

pub async fn get_todo(pool: &PgPool, user_id: i32) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(todos)
}

pub async fn get_todo_by_id(pool: &PgPool, user_id: i32, todo_id: i32) -> Result<Todo, String> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(todo_id)
        .fetch_one(pool)
        .await;

    match todo {
        Ok(todo) => Ok(todo),
        Err(sqlx::Error::RowNotFound) => Err(format!(
            "Todo not found either with id {} or not in db",
            todo_id
        )),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn update_todo(pool: &PgPool , user_id: i32, todo_id: i32, todo: &TodoData)-> Result<Todo, sqlx::Error>{
    let todo = sqlx::query_as::<_, Todo>("UPDATE TODOS SET title = $1, description = $2, tag = $3 WHERE id = $4 and user_id = $5 RETURNING *")
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(&todo.tag)
    .bind(todo_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn delete_todo(pool: &PgPool , user_id: i32, todo_id: i32)-> Result<Todo, sqlx::Error>{
    let todo = sqlx::query_as::<_, Todo>("DELETE FROM TODOS WHERE id = $1 and user_id = $2 RETURNING *")
    .bind(todo_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}
