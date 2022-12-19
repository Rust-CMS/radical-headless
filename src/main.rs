#[macro_use]
extern crate rocket;

mod api;
mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build()
}
