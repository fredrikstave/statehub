use bson::{Bson, doc, oid};
use mongodb::sync::Client;
use mongodb::error::Error;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topic {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<oid::ObjectId>,
    pub topicId: Option<String>,
    pub data: Option<JsonValue>
}

fn database_stuff() -> Result<Option<Topic>, Error> {
    match Client::with_uri_str("mongodb://root:rootpassword@localhost:27017") {
        Ok(client) => {
            let database = client.database("statehub");
            let collection = database.collection("topics");

            let docs = vec![
                doc! {
                    "topicId": "test/books/1984",
                    "data": {
                        "title": "1984",
                        "author": "George Orwell"
                    }
                },
                doc! {
                    "topicId": "test/books/animal-farm",
                    "data": {
                        "title": "Animal Farm",
                        "author": "George Orwell"
                    }
                },
                doc! {
                    "topicId": "test/books/the-great-gatsby",
                    "data": {
                        "title": "The Great Gatsby",
                        "author": "F. Scott Fitzgerald"
                    }
                },
            ];

            // Insert some documents into the "statehub.topics" collection.
            collection.insert_many(docs, None);

            match collection.find_one(Some(doc! { "topicId": "test/books/1984" }), None) {
                Ok(db_result) => {
                    match db_result {
                        Some(result_doc) => {
                            match bson::from_bson(bson::Bson::Document(result_doc)) {
                                Ok(result_model) => Ok(Some(result_model)),
                                Err(err) => Err(Error::from(err))
                            }
                        },
                        None => Ok(None)
                    }
                },
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(Error::from(err))
    }
}

#[get("/test-mongo")]
pub fn test_mongo() -> Result<Json<Topic>, Status> {
    match database_stuff() {
        Ok(res) => {
            match res {
                Some(data) => Ok(Json(data)),
                None => Err(Status::NotFound),
            }
        },
        Err(_) => Err(Status::NotFound)
    }
}