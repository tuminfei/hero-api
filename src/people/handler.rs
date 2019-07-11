use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};
use diesel::result::Error;
use std::env;

use crate::db::Connection;
use crate::schema::people;
use super::Person;
use crate::api::errors::handlers;

#[post("/", data = "<person>")]
pub fn create(conn: Connection, person: Json<Person>) -> Json<JsonValue> {
    let insert = Person {
        ..person.into_inner()
    };
    Person::create(&conn, &insert)
}

#[get("/")]
pub fn all(connection: Connection) -> Result<Json<Vec<Person>>, Status> {
    super::all(&connection)
        .map(|people| Json(people))
        .map_err(|error| handlers::error_status(error))
}
