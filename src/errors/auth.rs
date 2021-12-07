pub enum AuthError {
    // Error content would be the username
    PasswordMismatchError(String),
    // Database Error on Database Failure
    DatabaseError(diesel::result::Error),
    // Hash Error on Hash Failure
    HashError(bcrypt::BcryptError),
}
