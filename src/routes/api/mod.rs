pub mod ping;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API", |rocket| async {
        rocket.mount("/api", routes![ping::ping])
    })
}
