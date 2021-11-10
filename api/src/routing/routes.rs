use std::collections::HashMap;

use crate::extract::docs;
use mongodb::Client;
use warp::Filter;

fn with_db(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn mount_routes(
    client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("docs")
        .and(warp::get())
        .and(with_db(client))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(docs::list_all_doc_tags)
}
