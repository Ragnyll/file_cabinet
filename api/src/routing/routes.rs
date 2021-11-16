use crate::routing;
use mongodb::Client;
use warp::Filter;

pub fn mount_routes(
    client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    routing::docs::docs::docs_endpoints(client)
}

