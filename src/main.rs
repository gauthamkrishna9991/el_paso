// -- ALL LIBRARY IMPORTS --
// Database - Diesel
#[macro_use]
extern crate diesel;
// Serialization/Deserialization - Serde
#[macro_use]
extern crate serde;
// HTTP Server - Rocket
#[macro_use]
extern crate rocket;
// Hashing - BCrypt
extern crate bcrypt;
// Environment Variable - dotenv (.env)
extern crate dotenv;

// -- PACKAGE IMPORTS --
// Self-Imports
pub mod errors;
pub mod models;
pub mod schema;
pub mod routes;

use rocket_sync_db_pools::{database, diesel as dpool};

#[database("db")]
struct DB(dpool::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "EL_PASO SERVER"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::fairing())
        .attach(routes::stage())
        .mount("/", routes![index])
}
