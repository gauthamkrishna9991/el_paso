-- Users Data Type
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- TODO: Make sure that this is the only way you can store hashes for pwd(h+s)
-- if there's a better implementation, don't forget to use that.
CREATE TABLE IF NOT EXISTS users (
    user_id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL
);