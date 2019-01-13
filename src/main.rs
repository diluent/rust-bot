#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::Request;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn read() -> Json<JsonValue> {
    Json(json!({
        "success": true
    }))
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

fn main() {
    rocket::ignite()
        // .mount("/message", routes![create, update, delete])
        .mount("/messages", routes![read])
        .register(catchers![internal_error, not_found])
        .launch();
}
