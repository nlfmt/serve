use std::{path::PathBuf, str::FromStr};

use mime_guess::from_path;
use rocket::http::{ContentType, Status};

use crate::{assets::Assets, auth::AuthGuard};

#[get("/<path..>")]
pub async fn get_embedded_file(
    _auth: AuthGuard,
    path: PathBuf,
) -> Result<(ContentType, Vec<u8>), (Status, &'static str)> {
    let path = path.to_str().unwrap();
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime_type = from_path(path).first_or_octet_stream();
            Ok((
                ContentType::from_str(&mime_type.to_string()).unwrap(),
                content.data.to_vec(),
            ))
        }
        None => Err((Status::NotFound, "Not Found")),
    }
}