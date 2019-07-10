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

mod schema;
mod db;
pub mod api;
pub mod settings;
mod people;
mod hero;


use hero::{Hero, HeroPatch, HeroWithId};
use settings::Settings;


#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
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
        .mount("/", routes![hello])
        .mount("/hero",
               routes![hero::handler::create,
               hero::handler::update,
               hero::handler::delete,
               hero::handler::get_bulk,
               hero::handler::get_detail,
               hero::handler::patch],
        )
        .mount("/people",
               routes![people::handler::all,
                    people::handler::create],
        )
        .attach(options)
}

fn main() {
    // dotenv().ok();
    let settings = Settings::new().unwrap();
    rocket(settings).launch();
}
