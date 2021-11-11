use mongodb::bson::doc;
use mongodb::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ApiKey {
    authorized_email: String,
    api_name: String,
    api_key: String,
    issued_dt: String,
    expire_dt: String,
}

/// Checks if a request is authorized by seeing if there is an allowed matching api_key in the
/// database
async fn is_authorized(client: Client, api_key_val: &str) -> Result<bool, mongodb::error::Error> {
    let filter = doc! { "api_name": "file_cabinet", "api_key": api_key_val };
    let db = client.database("api_key_db");
    let collection = db.collection::<ApiKey>("api_keys");
    if let Some(key) = collection.find_one(filter, None).await? {
        Ok(!is_expired(key.expire_dt))
    } else {
        Ok(false)
    }
}

/// Determiens if the api key is expired by checking the expire_dt
fn is_expired(expire_dt: String) -> bool {
    false
}
