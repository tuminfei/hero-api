use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

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