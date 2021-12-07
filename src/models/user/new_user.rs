const SALT_SIZE: usize = 16;

use diesel::prelude::*;
use diesel::PgConnection;

use rand::{distributions::Alphanumeric, Rng};

use bcrypt::{hash_with_salt, DEFAULT_COST};

// - Schema + Model
//  - Model
use crate::schema::users;
//  - Schema Model
use crate::schema::users::dsl::users as users_schema;

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
    pub email: String,
    pub username: String,
    password_hash: String,
    password_salt: String,
}

impl NewUser {
    // CREATE OPERATION

    pub fn create_user(
        conn: &PgConnection,
        username: String,
        password: String,
        email: String,
    ) -> Result<User, AuthError> {
        // Create the new salt string
        let password_salt = get_salt_string();
        // Run hash with salt, with a default cost value.
        match hash_with_salt(password, DEFAULT_COST, password_salt.as_bytes()) {
            // If successful
            Ok(password_hash) => {
                // Create the new user for insertion
                let new_user = Self {
                    email,
                    username,
                    password_hash: password_hash.to_string(),
                    password_salt,
                };
                // Insert the new user, map error to database error
                diesel::insert_into(users_schema)
                    .values(&new_user)
                    .get_result(conn)
                    .map_err(AuthError::DatabaseError)
            }
            Err(hash_err) => Err(AuthError::HashError(hash_err)),
        }
    }
}
