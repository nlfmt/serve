use std::{
    fs,
    path::Path,
    time::UNIX_EPOCH,
};

use crate::models::{DirectoryContent, FileInfo, FolderInfo};

pub fn read_entries(path: &Path, follow_symlinks: bool) -> std::io::Result<DirectoryContent> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let mut meta = entry.metadata().ok()?;
            let is_symlink = meta.is_symlink();
            if is_symlink {
                meta = fs::metadata(entry.path()).ok()?;
            }
            let file_name = entry.file_name().into_string().ok()?;
            
            if is_symlink && !follow_symlinks {
                return None
            }

            Some((file_name, meta, is_symlink))
        })
        .fold(DirectoryContent::default(), |mut res, (name, meta, is_symlink)| {
            let size = meta.len();
            let modified = meta
                .modified()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let created = meta
                .created()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));

            if meta.is_dir() {
                res.dirs.push(FolderInfo {
                    name,
                    modified,
                    created,
                    is_symlink,
                })
            } else {
                res.files.push(FileInfo {
                    name,
                    size,
                    modified,
                    created,
                    is_symlink,
                });
            }
            res
        }))
}
