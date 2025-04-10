use rocket::{serde::json::Json, State};

use crate::{auth::AuthGuard, models::{AppState, Settings}};


#[get("/settings")]
pub async fn get_settings(_auth: AuthGuard, state: &State<AppState>) -> Json<Settings> {
    Json(Settings {
        upload: state.upload,
        overwrite: state.overwrite,
        delete: state.delete,
        rename: state.rename,
    })
}