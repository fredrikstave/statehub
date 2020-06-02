// #![allow(proc_macro_derive_resolution_fallback)]

// use crate::topics::{InsertableTopic, Topic};
// use crate::storage_engine::Conn;
// use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
// use mongodb::{error::Error, oid::ObjectId};
// use mongodb::{bson, doc};

// const COLLECTION: &str = "topics";

// pub fn all(connection: &Conn) -> Result<Vec<Topic>, Error> {
//     let cursor = connection.collection(COLLECTION).find(None, None).unwrap();

//     cursor
//         .map(|result| match result {
//             Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
//                 Ok(result_model) => Ok(result_model),
//                 Err(_) => Err(Error::DefaultError(String::from(""))),
//             },
//             Err(err) => Err(err),
//         })
//         .collect::<Result<Vec<Topic>, Error>>()
// }

// pub fn get(topic_id: String, connection: &Conn) -> Result<Option<Topic>, Error> {
//     match connection
//         .collection(COLLECTION)
//         .find_one(Some(doc! {"topicId": topic_id}), None)
//     {
//         Ok(db_result) => match db_result {
//             Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
//                 Ok(result_model) => Ok(Some(result_model)),
//                 Err(_) => Err(Error::DefaultError(String::from(
//                     "Failed to create reverse BSON",
//                 ))),
//             },
//             None => Ok(None),
//         },
//         Err(err) => Err(err),
//     }
// }

// pub fn insert(topic: Topic, connection: &Conn) -> Result<ObjectId, Error> {
//     let insertable = InsertableTopic::from_topic(topic.clone());
//     match bson::to_bson(&insertable) {
//         Ok(model_bson) => match model_bson {
//             bson::Bson::Document(model_doc) => {
//                 match connection
//                     .collection(COLLECTION)
//                     .insert_one(model_doc, None)
//                 {
//                     Ok(res) => match res.inserted_id {
//                         Some(res) => match bson::from_bson(res) {
//                             Ok(res) => Ok(res),
//                             Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
//                         },
//                         None => Err(Error::DefaultError(String::from("None"))),
//                     },
//                     Err(err) => Err(err),
//                 }
//             }
//             _ => Err(Error::DefaultError(String::from(
//                 "Failed to create Document",
//             ))),
//         },
//         Err(err) => {
//             println!("{:?}", err);
//             Err(Error::DefaultError(String::from("Failed to create BSON")))
//         },
//     }
// }