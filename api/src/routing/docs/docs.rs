use crate::data::{docs, create_docs};
use crate::routing::db::db_resource;
use mongodb::Client;
use std::collections::HashMap;
use warp::Filter;

pub fn docs_endpoints(
    db_client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_docs(db_client.clone())
        .or(post_doc(db_client.clone()))
}

pub fn get_docs(
    db_client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("docs")
        .and(warp::get())
        .and(db_resource::with_db(db_client))
        .and(warp::header("Authorization"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(docs::list_all_doc_tags)
}

pub fn post_doc(
    db_client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("docs")
        .and(warp::post())
        .and(db_resource::with_db(db_client))
        .and(warp::header("Authorization"))
        .and(create_docs::json_body())
        .and_then(create_docs::create_docs)
}
