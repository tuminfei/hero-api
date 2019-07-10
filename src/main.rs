#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]
#![feature(result_map_or_else)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate rocket_cors;
extern crate dotenv;
use dotenv::dotenv;
use rocket::Rocket;


#[macro_use]
extern crate diesel;
//use rocket_contrib::databases::diesel;

use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod hero;
mod schema;
mod db;
pub mod api;
pub mod settings;
mod people;


use hero::{Hero, HeroPatch, HeroWithId};
use settings::Settings;


#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<hero>")]
fn create(conn: db::Connection, hero: Json<Hero>) -> Json<JsonValue> {
    let insert = Hero {
        ..hero.into_inner()
    };
    Hero::create(&conn, &insert)
}

#[get("/")]
fn get_bulk(conn: db::Connection) -> Json<JsonValue> {
    Json(json!(Hero::get_bulk(&conn)))
}

#[get("/<id>")]
fn get_detail(conn: db::Connection, id: i32) -> Json<JsonValue> {
    Hero::get_detail(&conn, id)
}

#[put("/<id>", data = "<hero>")]
fn update(conn: db::Connection, id: i32, hero: Json<HeroWithId>) -> Json<JsonValue> {
    // TODO should they be updating *with* ID?
    let update = HeroWithId {
        ..hero.into_inner()
    };
    Hero::update(&conn, id, update)
}

#[patch("/<id>", data = "<hero>")]
fn patch(conn: db::Connection, id: i32, hero: Json<HeroPatch>) -> Json<JsonValue> {
    let update = HeroPatch {
        ..hero.into_inner()
    };
    Hero::patch(&conn, id, update)
}

#[delete("/<id>")]
fn delete(conn: db::Connection, id: i32) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(&conn, id)
    }))
}

fn rocket(settings: Settings) -> Rocket {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
    assert!(failed_origins.is_empty());
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Patch,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        //        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .manage(db::connect(&settings.database))
        .mount(
            "/hero",
            routes![create, update, delete, get_bulk, get_detail, patch],
        )
        .mount("/", routes![hello])
        .attach(options)
}

fn main() {
    // dotenv().ok();
    let settings = Settings::new().unwrap();
    people::router::create_routes();
    rocket(settings).launch();
}
