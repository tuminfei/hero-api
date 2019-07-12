use diesel::result::Error;
use std::env;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use crate::db::Connection;
use crate::schema::hero;
use crate::models::hero::{Hero, HeroPatch, HeroWithId};

#[post("/", data = "<hero>")]
pub fn create(conn: Connection, hero: Json<Hero>) -> Json<JsonValue> {
    let insert = Hero {
        ..hero.into_inner()
    };
    Hero::create(&conn, &insert)
}

#[get("/")]
pub fn get_bulk(conn: Connection) -> Json<JsonValue> {
    Json(json!(Hero::get_bulk(&conn)))
}

#[get("/<id>")]
pub fn get_detail(conn: Connection, id: i32) -> Json<JsonValue> {
    Hero::get_detail(&conn, id)
}

#[put("/<id>", data = "<hero>")]
pub fn update(conn: Connection, id: i32, hero: Json<HeroWithId>) -> Json<JsonValue> {
    // TODO should they be updating *with* ID?
    let update = HeroWithId {
        ..hero.into_inner()
    };
    Hero::update(&conn, id, update)
}

#[patch("/<id>", data = "<hero>")]
pub fn patch(conn: Connection, id: i32, hero: Json<HeroPatch>) -> Json<JsonValue> {
    let update = HeroPatch {
        ..hero.into_inner()
    };
    Hero::patch(&conn, id, update)
}

#[delete("/<id>")]
pub fn delete(conn: Connection, id: i32) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(&conn, id)
    }))
}
