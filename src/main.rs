#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod models;
pub mod schema;
mod util;

use diesel::PgConnection;

use models::*;

// #[get("/users/<id>")]
// fn get_user(connection: &PgConnection, id: i32) -> User {
//     users::get(connection, id)
// }

// #[post("/users", format = "application/json", data = "<user>")]
// fn create_user(user: NewUser, connection: PgConnection) -> User {
//     users::get(id, connection)
// }

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    let db_connection = db::connect();

    rocket::build().mount("/", routes![ping])
}
