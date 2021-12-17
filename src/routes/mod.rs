pub mod api;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Routes", |rocket| async { rocket.attach(api::stage()) })
}
