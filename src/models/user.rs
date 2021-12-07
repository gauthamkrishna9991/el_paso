use uuid::Uuid;

use diesel::prelude::*;
use diesel::PgConnection;

use rand::{distributions::Alphanumeric, Rng};

// - Schema + Model
//  - Model
use crate::schema::users;
//  - Schema Model
use crate::schema::users::dsl::users as users_schema;

// - bcrypt (hashing)
use bcrypt::{hash_with_salt, verify, DEFAULT_COST};

// Size for Salt Generation
const SALT_SIZE: usize = 16;

#[must_use]
pub fn get_salt_string() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(SALT_SIZE)
        .map(char::from)
        .collect()
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    //  Make that you switch to a better data type, better than to handle
    //  password salts and hashes as a string.
    password_hash: String,
    password_salt: String,
}

pub enum AuthError {
    // Error content would be the username
    PasswordMismatchError(String),
    DatabaseError(diesel::result::Error),
    HashError(bcrypt::BcryptError),
}

impl User {
    // READ OPERATIONS

    pub fn get_users(
        conn: &PgConnection,
        skip: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<User>, diesel::result::Error> {
        users_schema
            .offset(skip.unwrap_or(0))
            .limit(limit.unwrap_or(20))
            .load::<User>(conn)
    }

    pub fn get_user(conn: &PgConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
        users_schema.find(user_id).first(conn)
    }
    pub fn get_user_by_email(
        conn: &PgConnection,
        email: String,
    ) -> Result<User, diesel::result::Error> {
        users_schema
            .filter(users::email.eq(email))
            .first::<User>(conn)
    }

    pub fn auth_uname(
        conn: &PgConnection,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        match users_schema
            .filter(users::username.eq(username.clone()))
            .first::<User>(conn)
        {
            Ok(user) => match verify(password, &user.password_hash) {
                Ok(val) => {
                    if val {
                        Ok(user)
                    } else {
                        Err(AuthError::PasswordMismatchError(username))
                    }
                }
                Err(hash_err) => Err(AuthError::HashError(hash_err)),
            },
            Err(db_err) => Err(AuthError::DatabaseError(db_err)),
        }
    }

    // UPDATE OPERATION

    pub fn update(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        diesel::update(users_schema.find(self.user_id))
            .set((
                users::email.eq(self.email.clone()),
                users::username.eq(self.username.clone()),
                users::password_hash.eq(self.password_hash.clone()),
                users::password_salt.eq(self.password_salt.clone()),
            ))
            .execute(conn)
            .map(|ans| ans == 1)
    }

    // DELETE OPERATION

    pub fn delete(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        diesel::delete(users_schema.find(self.user_id))
            .execute(conn)
            .map(|ans| ans == 1)
    }
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
                let new_user = NewUser {
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
