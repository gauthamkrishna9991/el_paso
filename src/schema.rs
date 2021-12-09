table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        password_hash -> Text,
        password_salt -> Text,
    }
}
