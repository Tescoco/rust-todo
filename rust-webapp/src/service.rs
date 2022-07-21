use crate::db;
use crate::models::{NewTodo, Todo};
use crate::schema::todos;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create(new_todo: NewTodo) -> Todo {
    let connection = db::create_connection();
    connection
        .transaction::<Todo, Error, _>(|| {
            let todo: Todo = diesel::insert_into(todos::table)
                .values(&new_todo)
                .get_result(&connection)
                .unwrap();
            Ok(todo)
        })
        .unwrap()
}
