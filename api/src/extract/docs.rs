use mongodb::Client;
use futures::stream::TryStreamExt;
use std::collections::HashMap;

use crate::extract::models::doc;
use crate::auth::authorization;

pub async fn list_all_doc_tags(
    client: Client,
    auth: String,
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    if !authorization::is_authorized(&client, &auth)
        .await
        .expect("unable to handle authorization")
    {}

    let db = client.database("tagdb");
    let collection = db.collection::<doc::Doc>("docs");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs = vec![];
    // Iterate over the results of the cursor.
    while let Some(my_doc) = cursor.try_next().await.unwrap() {
        docs.push(my_doc);
    }
    println!("{:?}", param);
    Ok(warp::reply::json(&docs))
}
