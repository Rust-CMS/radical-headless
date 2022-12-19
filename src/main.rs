use diesel::{PgConnection, Connection};
use std::env;
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

mod api;
mod controllers;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATBASE_URL not set.");
    PgConnection::establish(&database_url)
        .unwrap()
}

#[launch]
fn rocket() -> _ {
    
    rocket::build()
}
