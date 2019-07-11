use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use super::io::*;
use diesel::result::Error as DieselError;
use rocket::request::Request;
use chrono::Local;

fn get_status_code_from_diesel_err(e: DieselError) -> i32 {
    if e == DieselError::NotFound {
        404
    } else {
        400
    }
}

pub fn diesel_err_to_json(e: DieselError) -> Json<JsonValue> {
    Json(json!({
        "error": e.to_string(),
        "status_code": get_status_code_from_diesel_err(e),
    }))
}

pub fn error_status(error: DieselError) -> Status {
    match error {
        DieselError::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 400,
            message: String::from("Incomplete Or Invalid Parameter Exception"),
            message_shortcode: String::from("incomplete_or_invalid_parameter"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("IncompleteOrInvalidParameterException"),
        },
    })
}

#[catch(401)]
pub fn unauthenticated(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 401,
            message: String::from("Authorization Required"),
            message_shortcode: String::from("unauthenticated"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("AuthorizationRequired"),
        },
    })
}

#[catch(403)]
pub fn unauthorized(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 403,
            message: String::from("Unauthorized"),
            message_shortcode: String::from("unauthorized"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("UnauthorizedException"),
        },
    })
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 404,
            message: String::from("Not Found"),
            message_shortcode: String::from("not_found"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("NotFound"),
        },
    })
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 422,
            message: String::from("Unprocessable Entity"),
            message_shortcode: String::from("unprocessable_entity"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("UnprocessableEntity"),
        },
    })
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 500,
            message: String::from("Internal Server Error"),
            message_shortcode: String::from("internal_server_error"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            // url: String::from(req.uri().as_str()),
            error_type: String::from("InternalServerError"),
        },
    })
}
