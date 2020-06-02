// use crate::topics;
// use crate::storage_engine::Conn;
// use topics::Topic;
// use mongodb::{error::Error, oid::ObjectId};
// use rocket::http::Status;
// use rocket_contrib::json::{Json, JsonValue};
// use std::path::PathBuf;

// fn error_status(error: Error) -> Status {
//     match error {
//         Error::CursorNotFoundError => Status::NotFound,
//         _ => Status::InternalServerError,
//     }
// }

// #[get("/state")]
// pub fn all(connection: Conn) -> Result<Json<Vec<Topic>>, Status> {
//     match topics::repository::all(&connection) {
//         Ok(res) => Ok(Json(res)),
//         Err(err) => Err(error_status(err)),
//     }
// }

// #[get("/state/<topic_id..>")]
// pub fn get_state(topic_id: PathBuf, connection: Conn) -> Result<Json<Topic>, Status> {
//     if let Some(topic_id) = topic_id.to_str() {
//         Ok(Json({
//             "status": "success",
//             "topic": topic_id
//         }))
//     } else {
//         Err(Status::InternalServerError)
//     }
// }

// #[post("/state/<topic_id..>", format = "json", data = "<state>")]
// pub fn store_state(topic_id: PathBuf, state: Json<Topic>, connection: Conn) -> Result<JsonValue, Status> {
//     match topics::repository::insert(state.into_inner(), &connection) {
//         Ok(res) => Ok(json!(res)),
//         Err(err) => {
//             println!("{:?}", err);
//             Err(error_status(err))
//         },
//     }
// }
