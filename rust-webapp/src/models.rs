use crate::schema::todos;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Associations, Identifiable, Serialize)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub checked: bool,
}

#[derive(Insertable, Deserialize, Associations)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: Option<String>,
    pub checked: bool,
}
