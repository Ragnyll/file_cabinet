use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::json;

#[derive(Serialize)]
struct GenericResponse {
    message: String,
    status: u16,
}

impl GenericResponse {
    fn new(message: &str, status_code: StatusCode) -> GenericResponse {
        GenericResponse {
            message: String::from(message),
            status: status_code.as_u16(),
        }
    }
}

/// Provides the server reply given the status and optional data
pub fn handle_response<T: Serialize>(
    status_code: StatusCode,
    data: Option<T>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match status_code {
        StatusCode::OK => Ok(warp::reply::with_status(json(&data.unwrap()), status_code)),
        StatusCode::FORBIDDEN => Ok(warp::reply::with_status(
            json(&GenericResponse::new("Forbidden", status_code)),
            status_code,
        )),
        StatusCode::CREATED => Ok(warp::reply::with_status(
            json(&GenericResponse::new("created", status_code)),
            status_code,
        )),
        _ => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(warp::reply::with_status(
                json(&GenericResponse::new("Internal Server Error", code)),
                code,
            ))
        }
    }
}
