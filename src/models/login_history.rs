use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::{DateTime, Utc};
use time::now;
use chrono::NaiveDateTime;

use crate::schema::login_history;
use crate::schema::login_history::dsl::*;
use crate::models::user::User;

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(un: &str, conn: &MysqlConnection) -> Option<LoginHistoryInsertableDTO> {
        if let Some(user) = User::find_user_by_username(un, conn) {
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: Utc::now().naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertableDTO, conn: &MysqlConnection) -> bool {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(conn)
            .is_ok()
    }
}