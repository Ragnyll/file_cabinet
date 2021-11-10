use warp::Filter;

use serde::{Serialize, Deserialize};
use mongodb::Client;
use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
    path: Option<String>,
    inode: f32,
    tags: Vec<String>,
}

fn with_db(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

async fn list_all_doc_tags(
    client: Client,
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let db = client.database("tagdb");
    let collection = db.collection::<Doc>("docs");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs = vec![];
    // Iterate over the results of the cursor.
    while let Some(my_doc) = cursor.try_next().await.unwrap() {
        docs.push(my_doc);
    }
    println!("{:?}", param);
    Ok(warp::reply::json(&docs))
}

/// Mounts all the routes required to run the api
pub fn mount_routes(
    client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("docs")
        .and(warp::get())
        .and(with_db(client))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(list_all_doc_tags)
}
