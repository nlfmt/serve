use rocket::{fs::NamedFile, http::Status, State};

use crate::{auth::AuthGuard, models::{AppState, FileResponse}, util::path::{parse_relative_path, pretty_path}};

#[get("/download?<path>")]
pub async fn download_file(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<FileResponse, (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.allow_symlinks) {
        Some(path) => {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            println!(
                "download \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
                &file_name,
                pretty_path(path.parent().unwrap())
            );
            let file = NamedFile::open(&path).await.unwrap();
            Ok(FileResponse {
                inner: file,
                file_name,
            })
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}