use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::Connection;
use crate::jwt::UserToken;
use crate::schema::people;
use crate::api::repo::people_repo;
use crate::api::errors::handlers;
use crate::models::response::Response;

#[get("/")]
pub fn find_all(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: Connection,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = people_repo::find_all(&conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}