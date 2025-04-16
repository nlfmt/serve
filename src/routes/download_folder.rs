use std::{fs::File, path::Path};

use rocket::{fs::NamedFile, http::Status, State};
use walkdir::WalkDir;

use crate::{auth::AuthGuard, log_error, models::FileResponse, state::AppState, util::{path::{generate_temp_path, parse_relative_path, pretty_path}, zip::zip_dir}};
use crate::color::{BLUE, LBLUE};

#[get("/download_folder?<path>")]
pub async fn download_folder(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<FileResponse, (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            let tmp_file_path = generate_temp_path();
            let tmp_path = Path::new(&tmp_file_path);
            let tmp_file = File::create(tmp_path).unwrap();

            let walkdir = WalkDir::new(&path).follow_links(false);
            let it = walkdir.into_iter();

            match zip_dir(&mut it.filter_map(|e| e.ok()), &path, tmp_file) {
                Ok(_) => {
                    let file = NamedFile::open(tmp_path).await.unwrap();
                    let file_name = format!("{}.zip", path.file_name().unwrap().to_str().unwrap());
                    println!(
                        "{BLUE}download zip {LBLUE}{}\x1b[0m",
                        pretty_path(&path)
                    );
                    Ok(FileResponse { inner: file, file_name })
                }
                Err(e) => {
                    log_error!("Could not create zip file: {e}");
                    Err((Status::InternalServerError, "Could not create zip file"))
                }
            }
        }
        None => Err((Status::BadRequest, "Invalid Path"))
    }
}
