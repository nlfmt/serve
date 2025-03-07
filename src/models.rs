use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub struct AppState {
    pub file_dir: PathBuf,
}

#[derive(Serialize)]
pub struct DirectoryContent {
    pub dirs: Vec<String>,
    pub files: Vec<String>,
}
impl Default for DirectoryContent {
    fn default() -> Self {
        DirectoryContent {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct FilesQuery {
    pub path: String,
}

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub path: String,
}
