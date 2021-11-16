use mongodb::Client;
use warp::Filter;

/// Utility used to to attaching the mongodb client to endpoints
pub fn with_db(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
