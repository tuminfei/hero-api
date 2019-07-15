use diesel;
use diesel::mysql::MysqlConnection;
use rocket::http::Status;

use crate::constants::message_constants;
use crate::jwt;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::people::{Person};

pub fn find_all(connection: &MysqlConnection) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(message_constants::MESSAGE_OK),
            data: serde_json::to_value(Person::find_all(&connection)).unwrap(),
        },
    }
}