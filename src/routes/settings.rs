use rocket::State;

use crate::{auth::AuthGuard, models::AppState};

#[get("/upload_enabled")]
pub async fn get_upload_enabled(_auth: AuthGuard, state: &State<AppState>) -> String {
    state.allow_upload.to_string()
}