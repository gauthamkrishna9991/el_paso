#[get("/")]
pub fn ping() -> &'static str {
    "Pong!"
}
