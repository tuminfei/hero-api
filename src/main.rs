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
extern crate chrono;
use dotenv::dotenv;
use rocket::Rocket;


#[macro_use]
extern crate diesel;
//use rocket_contrib::databases::diesel;

#[macro_use]
extern crate log;
extern crate env_logger;

use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod schema;
mod db;
mod jwt;
pub mod api;
pub mod settings;
pub mod constants;
pub mod models;
mod people;

use settings::Settings;
use log::Level;
use api::errors::handlers::*;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}



fn rocket(settings: Settings) -> Rocket {
    let allowed_origins = &settings.app.allowed_origins.clone();
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[allowed_origins]);
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
        .mount("/", routes![hello])
        .mount("/user/auth",
               routes![api::controllers::user_controller::login,
               api::controllers::user_controller::signup])
        .mount("/hero",
               routes![api::controllers::hero_controller::create,
               api::controllers::hero_controller::update,
               api::controllers::hero_controller::delete,
               api::controllers::hero_controller::get_bulk,
               api::controllers::hero_controller::get_detail,
               api::controllers::hero_controller::patch],
        )
        .mount("/people",
               routes![people::handler::all,
                    people::handler::create],
        )
        .register(catchers![
            not_found,
            unauthenticated,
            unauthorized,
            bad_request,
            unprocessable_entity,
            internal_server_error
        ])
        .attach(options)
}

fn main() {
    // dotenv().ok();
    let settings = Settings::new().unwrap();
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, settings.log.filter.clone());
    env_logger::Builder::from_env(env).init();

    rocket(settings).launch();
}
