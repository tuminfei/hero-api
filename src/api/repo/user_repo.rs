use diesel;
use diesel::mysql::MysqlConnection;
use rocket::http::Status;

use crate::constants::message_constants;
use crate::jwt;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};

pub fn signup(user: UserDTO, connection: &MysqlConnection) -> ResponseWithStatus {
    if User::signup(user, &connection) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn login(login: LoginDTO, connection: &MysqlConnection) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &connection) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(json!({ "token": jwt::generate_token(result), "type": "bearer" }))
                    .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}