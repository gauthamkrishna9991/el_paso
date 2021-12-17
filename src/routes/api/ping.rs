use rocket::serde::json::Json;


#[derive(Serialize)]
pub struct PingResult {
    status: bool,
    message: String,
}

impl PingResult {
    pub fn create() -> Self {
        Self {
            status: true,
            message: "Success".to_string(),
        }
    }
}

#[get("/")]
pub fn ping() -> Json<PingResult> {
    Json::from(PingResult::create())
}
