use crate::people;
use rocket;
use crate::db;

pub fn create_routes() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/people",
               routes![people::handler::all,
                    people::handler::create],
        );
}
