use rocket::State;

use crate::models::AppState;

#[get("/upload_enabled")]
pub async fn get_upload_enabled(state: &State<AppState>) -> String {
    state.allow_upload.to_string()
}