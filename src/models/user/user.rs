//! User Models
//!
//! All Models needed for user operations

// UUID Support
use uuid::Uuid;

//  - Database Library Imports
//    - Diesel Prelude
use diesel::prelude::*;
//    - Connection
use diesel::PgConnection;

//  - Datetime/Timestamp Imports
use std::time::SystemTime;

//  - Schema + Model
//    - Model
use crate::schema::users;
//    - Schema
use crate::schema::users::dsl::users as users_schema;

// - bcrypt (hashing)
use bcrypt::verify;

// Authentication Error
use crate::errors::auth::AuthError;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    //  Make that you switch to a better data type, better than to handle
    //  password salts and hashes as a string.
    password_hash: String,
    password_salt: String,
    date_created: SystemTime,
    last_updated: SystemTime,
}

impl User {
    // READ OPERATIONS

    pub fn get_users(
        conn: &PgConnection,
        skip: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        users_schema
            .offset(skip.unwrap_or(0))
            .limit(limit.unwrap_or(20))
            .load::<Self>(conn)
    }

    pub fn get_user(conn: &PgConnection, user_id: Uuid) -> Result<Self, diesel::result::Error> {
        users_schema.find(user_id).first(conn)
    }
    // pub fn get_user_by_email(
    //     conn: &PgConnection,
    //     email: String,
    // ) -> Result<Self, diesel::result::Error> {
    //     users_schema.filter(users::email.eq(email)).first(conn)
    // }

    pub fn auth_uname(
        conn: &PgConnection,
        username: String,
        password: String,
    ) -> Result<Self, AuthError> {
        match users_schema
            .filter(users::username.eq(username.clone()))
            .first::<Self>(conn)
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
                users::username.eq(self.username.clone()),
                users::password_hash.eq(self.password_hash.clone()),
                users::password_salt.eq(self.password_salt.clone()),
                users::last_updated.eq(SystemTime::now()),
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
