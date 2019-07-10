use crate::people;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .mount("/people",
               routes![people::handler::all,
                    people::handler::create],
        );
}
