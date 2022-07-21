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

pub fn get_all() -> Vec<Todo> {
    let connection = db::create_connection();
    todos::table.load::<Todo>(&connection).unwrap()
}

pub fn update_todo(todo: Todo) -> Todo {
    let connection = db::create_connection();
    diesel::update(todos::table.find(todo.id))
        .set(todos::columns::title.eq(todo.title))
        .get_result(&connection)
        .unwrap()
}

pub fn delete_todo(id: i32) {
    let connection = db::create_connection();
    diesel::delete(todos::table.find(id))
        .execute(&connection)
        .unwrap();
}
