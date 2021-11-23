use crate::data::models::{doc, tags};
use crate::auth::authorization;
use crate::server::reply;
use mongodb::Client;
use warp::Filter;
use warp::http::StatusCode;

const MAX_BODY_SIZE: u64 = 1024 * 16;

/// accepts a json body of docs up to MAX_BODY_SIZE
pub fn json_body() -> impl Filter<Extract = (tags::Tags,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_BODY_SIZE).and(warp::body::json())
}

/// adds the tags to the doc specified by the inode
pub async fn add_tags(
    client: Client,
    auth: String,
    body: tags::Tags,
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

    println!("LETS DO A PATCH with tags {:?}", body);
    reply::handle_response(StatusCode::CREATED, Some("_"))
}
