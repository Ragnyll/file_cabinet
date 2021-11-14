use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::json;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    status: u16,
}

impl ErrorResponse {
    fn new(message: &str, status_code: StatusCode) -> ErrorResponse {
        ErrorResponse {
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
        StatusCode::OK => {
            // TODO: change this unwrap to provide a default response
            Ok(warp::reply::with_status(json(&data.unwrap()), status_code))
        }
        StatusCode::FORBIDDEN => Ok(warp::reply::with_status(
            json(&ErrorResponse::new("Forbidden", status_code)),
            status_code,
        )),

        _ => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(warp::reply::with_status(
                json(&ErrorResponse::new("Internal Server Error", code)),
                code,
            ))
        }
    }
}
