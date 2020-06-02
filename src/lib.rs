#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate dotenv;
extern crate mongodb;
extern crate serde_json;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use std::path::PathBuf;
use rocket_contrib::json::{Json, JsonValue};

mod storage_engine;

#[derive(Serialize, Deserialize, Debug)]
struct Schema {
    foo: String,
    bar: String
}

#[get("/")]
fn index() -> JsonValue {
    json!({
        "status": "info",
        "reason": "Welcome to StateHub"
    })
}

#[post("/define-schema/<topic..>", format = "json", data = "<schema>")]
fn define_schema(topic: PathBuf, schema: Json<Schema>) -> JsonValue {
    json!({
        "status": "ok",
        "function": "define_schema",
        "topic": topic,
        "schema": format!("{:?}", schema)
    })
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Sorry, we don\'t allow that."
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn internal_error() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Something went horribly wrong."
    })
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .register(catchers![bad_request, internal_error, not_found])
        .mount("/define-schema", routes![define_schema])
        // .mount("/", routes![
        //     index,
        //     topics::handler::all,
        //     topics::handler::get_state,
        //     topics::handler::store_state
        // ])
}