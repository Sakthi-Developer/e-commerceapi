use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse <T> where T: Serialize {
    pub status: String,
    pub msg: String,
    pub data: T,
}