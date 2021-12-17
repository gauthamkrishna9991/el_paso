// Size of random salt characters addeable.
const SALT_SIZE: usize = 16;

// Hashing - bcrypt
use bcrypt::{hash_with_salt, DEFAULT_COST};

// Random Value Generation - rand
use rand::{distributions::Alphanumeric, Rng};

//  - Database Library Imports
//    - Diesel Prelude
use diesel::prelude::*;
//    - Connection
use diesel::PgConnection;

//  - Datetime/Timestamp Library
use std::time::SystemTime;

//  - Schema + Model
//    - Model
use crate::schema::users;
//    - Schema
use crate::schema::users::dsl::users as users_schema;

// Import User Type
use super::user::User;

// Authentication Error
use crate::errors::auth::AuthError;

#[must_use]
pub fn get_salt_string() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(SALT_SIZE)
        .map(char::from)
        .collect()
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    password_hash: String,
    password_salt: String,
    date_created: SystemTime,
<<<<<<< HEAD
    last_modified: SystemTime,
=======
    last_updated: SystemTime,
>>>>>>> ff063eb0459f414334c3816682f40967cb26abde
}

impl NewUser {
    // CREATE OPERATION

    pub fn create_user(
        conn: &PgConnection,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        // Create the new salt string
        let password_salt = get_salt_string();
        // Run hash with salt, with a default cost value.
        match hash_with_salt(password, DEFAULT_COST, password_salt.as_bytes()) {
            // If successful
            Ok(password_hash) => {
                // Create the new user for insertion
                let new_user = Self {
                    username,
                    password_hash: password_hash.to_string(),
                    password_salt,
                    date_created: SystemTime::now(),
<<<<<<< HEAD
                    last_modified: SystemTime::now(),
=======
                    last_updated: SystemTime::now(),
>>>>>>> ff063eb0459f414334c3816682f40967cb26abde
                };
                // Insert the new user, map error to database error
                diesel::insert_into(users_schema)
                    .values(&new_user)
                    .get_result::<User>(conn)
                    .map_err(AuthError::DatabaseError)
            }
            // If unsuccessful
            Err(hash_err) => Err(AuthError::HashError(hash_err)),
        }
    }
}
