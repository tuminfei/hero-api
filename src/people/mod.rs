#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket_contrib::json::{Json, JsonValue};

pub mod handler;
pub mod router;

use super::schema;
use schema::people;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Insertable)]
#[table_name = "people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub profession: String,
    pub salary: i32,
}

fn get_status_code_from_diesel_err(e: DieselError) -> i32 {
    if e == DieselError::NotFound {
        404
    } else {
        400
    }
}

fn diesel_err_to_json(e: DieselError) -> Json<JsonValue> {
    Json(json!({
        "error": e.to_string(),
        "status_code": get_status_code_from_diesel_err(e),
    }))
}

pub fn all(connection: &diesel::MysqlConnection) -> QueryResult<Vec<Person>> {
    people::table.load::<Person>(&*connection)
}

impl Person {
    fn get_most_recently_created_person(connection: &diesel::MysqlConnection) -> Person {
        people::table
            .order(people::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn create(connection: &diesel::MysqlConnection, p: &Person) -> Json<JsonValue>{
        diesel::insert_into(people::table)
            .values(p)
            .execute(connection)
            .map_or_else(
                |e| diesel_err_to_json(e),
                |res| Json(json!(Person::get_most_recently_created_person(connection))),
            )
    }
}