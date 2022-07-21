// #[macro_use]
// extern crate rocket;
// #[macro_use]
// extern crate diesel;
// mod db;
// mod schema;
// use crate::schema::todo;
// use diesel::{Insertable, Queryable};
// use rocket::fs::{relative, FileServer};
// use rocket::serde::json::Json;
// use rocket::{get, post, routes};
// use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod db;
mod models;
mod schema;
use crate::models::{NewTodo, Todo};
use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;
mod service;

#[get("/test")]
fn index() -> &'static str {
    "hello world"
}

//create
#[post("/todo/create", data = "<payload>")]
fn save_todo(payload: Json<NewTodo>) -> Json<Todo> {
    let new_todos = payload.into_inner();
    Json(service::create(new_todos))
}

//read all
#[get("/todo/getall")]
fn get_all_todo() -> Json<Vec<Todo>> {
    Json(service::get_all())
}

//update
#[put("/todo/update", data = "<payload>")]
fn update_todo(payload: Json<Todo>) -> Json<Todo> {
    Json(service::update_todo(payload.into_inner()))
}

//delete
#[delete("/todo/delete/<id>")]
fn delete_todo(id: i32) {
    service::delete_todo(id);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("client/build")))
        .mount(
            "/api",
            routes![index, save_todo, get_all_todo, update_todo, delete_todo],
        )
}
