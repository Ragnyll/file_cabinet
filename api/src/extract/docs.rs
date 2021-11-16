use crate::extract::models::doc;
use crate::auth::authorization;
use crate::server::reply;
use futures::stream::TryStreamExt;
use mongodb::Client;
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn list_all_doc_tags(
    client: Client,
    auth: String,
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    // TODO: move this expct call into something that will 500
    match authorization::is_authorized(&client, &auth)
        .await {
            Ok(r) => {
                if !r {
                    return reply::handle_response(StatusCode::FORBIDDEN, None);
                }
            },
            Err(_) => {
                return reply::handle_response(StatusCode::INTERNAL_SERVER_ERROR, None);
            }
        }

    let db = client.database("tagdb");
    let collection = db.collection::<doc::Doc>("docs");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs = vec![];
    // Iterate over the results of the cursor.
    while let Some(my_doc) = cursor.try_next().await.unwrap() {
        docs.push(my_doc);
    }

    reply::handle_response(StatusCode::OK, Some(docs))
}
