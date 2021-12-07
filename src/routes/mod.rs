pub mod api;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API", |rocket| async { rocket })
}
