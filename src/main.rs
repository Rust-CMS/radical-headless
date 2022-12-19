use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate rocket;

mod api;
mod controllers;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATBASE_URL not set.");
    PgConnection::establish(&database_url).unwrap()
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
