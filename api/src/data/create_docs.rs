use crate::data::models::doc;
use crate::auth::authorization;
use crate::server::reply;
use mongodb::Client;
use warp::Filter;
use warp::http::StatusCode;

const MAX_BODY_SIZE: u64 = 1024 * 16;

/// accepts a json body of docs up to MAX_BODY_SIZE
pub fn json_body() -> impl Filter<Extract = (doc::Doc,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_BODY_SIZE).and(warp::body::json())
}

/// Creates a doc based off the body
pub async fn create_docs(
    client: Client,
    auth: String,
    body: doc::Doc,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match authorization::is_authorized(&client, &auth).await {
        Ok(r) => {
            if !r {
                return reply::handle_response(StatusCode::FORBIDDEN, None);
            }
        }
        Err(_) => {
            return reply::handle_response(StatusCode::INTERNAL_SERVER_ERROR, None);
        }
    }

    let db = client.database("tagdb");
    let collection = db.collection::<doc::Doc>("docs");

    match collection.insert_one( body, None).await {
        // _ is a place holder for T that will be discarded
        Ok(_) => reply::handle_response(StatusCode::CREATED, Some("_")),
        Err(_) => reply::handle_response(StatusCode::INTERNAL_SERVER_ERROR, None),
    }
}
