
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_id: i32,
    pub task_name: String,
    pub is_done: i32,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub task_remain: i32,
    pub task_done: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTask {
    pub user_id: String,
    pub task_name: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUser {
    pub user_id: String,
    pub user_name: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskAction{
    pub user_id : String,
    pub task_id : i32
}