use serde::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Deserialize, Serialize, Queryable, Identifiable, Associations)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub status: i32
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    title: String,
    status: i32
}