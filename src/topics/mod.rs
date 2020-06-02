#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
use bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topic {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub topicId: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTopic {
    pub topicId: Option<String>
}

impl InsertableTopic {
    fn from_topic(topic: Topic) -> InsertableTopic {
        InsertableTopic {
            topicId: topic.topicId,
        }
    }
}