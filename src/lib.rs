// Schemas + Models
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
extern crate bcrypt;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[must_use]
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
