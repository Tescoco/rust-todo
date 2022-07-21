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

#[post("/todo/create", data = "<payload>")]
fn save_todo(payload: Json<NewTodo>) -> Json<Todo> {
    let new_todos = payload.into_inner();
    Json(service::create(new_todos))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("client/build")))
        .mount("/api", routes![index, save_todo])
}
