use crate::db::Connection;
use crate::models::response::Response;
use crate::models::user::{LoginDTO, UserDTO};
use crate::api::repo::user_repo;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserDTO>, conn: Connection) -> status::Custom<Json<Response>> {
    let response = user_repo::signup(user.0, &conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginDTO>, conn: Connection) -> status::Custom<Json<Response>> {
    let response = user_repo::login(login.0, &conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}