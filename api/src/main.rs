#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use diesel::{SqliteConnection, Connection, RunQueryDsl};

use rocket::{Request, Response};
use rocket::fairing::{Info, Kind, Fairing};
use rocket::http::{Header, Status};
use rocket::{State, serde::json::Json};

use std::sync::Mutex;
use todo::NewTodo;

mod schema;
mod todo;

use schema::todos;
use crate::todo::Todo;

#[get("/todos")]
fn get_todos(database: &Database) -> Json<Vec<Todo>> {
    let db = &*database.lock().unwrap();
    let result: Vec<Todo> = todos::dsl::todos.load(db).unwrap();

    Json::from(result)
}

#[post("/todo", data = "<todo>")]
fn post_todo(todo: Json<NewTodo>, database: &Database) {
    let db = &*database.lock().unwrap();
    diesel::insert_into(todos::table).values(&todo.0).execute(db).expect("Could not insert Todo");
}

#[delete("/todo?<id>")]
fn delete_todo(id: i32, database: &Database) {
    let db = &*database.lock().unwrap();
    diesel::delete(todos::table.filter(todos::id.eq(id))).execute(db).expect("Could not delete Todo");
}

#[post("/changeTodo", data = "<todo>")]
fn put_todo(todo: Json<Todo>, database: &Database) {
    let db = &*database.lock().unwrap();
    let todo = todo.0;
    diesel::update(todos::table.filter(todos::id.eq(todo.id)))
                   .set((todos::status.eq(todo.status), todos::title.eq(todo.title)))
                   .execute(db).expect("Could not update Todo");
}

#[options("/todo")]
fn todo_option() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let connection = SqliteConnection::establish("db.sqlite").unwrap();

    rocket::build()
    .attach(CORS)
    .manage(Mutex::from(connection))
    .mount("/api", routes![
        get_todos,
        post_todo,
        delete_todo,
        put_todo,
        todo_option
    ])
}

type Database = State<Mutex<SqliteConnection>>;

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}