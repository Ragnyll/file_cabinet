use serde::{Serialize, Deserialize};
use mongodb::bson::doc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tags {
    tags: Vec<String>,
}
