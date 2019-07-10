use crate::db::Connection;
use diesel::result::Error;
use std::env;
use crate::schema::people;
use super::Person;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

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
        .map_err(|error| error_status(error))
}
