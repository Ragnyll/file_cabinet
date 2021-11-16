use chrono::DateTime;
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Serialize, Deserialize};

const DT_FORMAT: &str = "%Y %b %d %H:%M:%S%.3f %z";

#[derive(Serialize, Deserialize, Debug)]
struct ApiKey {
    authorized_email: String,
    api_name: String,
    api_key: String,
    issued_dt: String,
    /// consumes dt of format "%Y %b %d %H:%M:%S%.3f %z"
    expire_dt: String,
}

/// Checks if a request is authorized by seeing if there is an allowed matching api_key in the
/// database
pub async fn is_authorized(client: &Client, api_key_val: &str) -> Result<bool, mongodb::error::Error> {
    let filter = doc! { "api_name": "file_cabinet", "api_key": api_key_val };
    let db = client.database("api_key_db");
    let collection = db.collection::<ApiKey>("api_keys");
    if let Some(key) = collection.find_one(filter, None).await? {
        Ok(!is_expired(key.expire_dt).expect("Unable to determine key expire_dt validity"))
    } else {
        Ok(false)
    }
}

/// Determines if the api key is expired by checking the expire_dt
fn is_expired(expire_dt: String) -> Result<bool, chrono::ParseError> {
    Ok(DateTime::parse_from_str(&expire_dt, DT_FORMAT)? < chrono::offset::Utc::now())
}
