// Schemas + Models
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
extern crate bcrypt;
extern crate dotenv;

use rocket_sync_db_pools::{database, diesel as dpool};

#[database("db")]
struct DB(dpool::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::fairing())
        .mount("/", routes![index])
}
