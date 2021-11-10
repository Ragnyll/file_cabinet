use serde::{Serialize, Deserialize};
use mongodb::bson::{doc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
    path: Option<String>,
    inode: f32,
    tags: Vec<String>,
}
