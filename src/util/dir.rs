use std::{
    fs,
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use crate::models::{DirectoryContent, FileInfo, FolderInfo};

fn follow_symlink(link: &Path) -> std::io::Result<PathBuf> {
    if fs::symlink_metadata(&link)?.file_type().is_symlink() {
        let target = fs::read_link(link)?;
        let meta = fs::metadata(&target)?;
        if meta.is_symlink() {
            follow_symlink(&target)
        } else {
            Ok(target)
        }
    } else {
        Ok(link.to_path_buf())
    }
}

pub fn read_entries(path: &Path, follow_symlinks: bool) -> std::io::Result<DirectoryContent> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let mut meta = entry.metadata().ok()?;
            if meta.is_symlink() {
                if follow_symlinks {
                    meta = fs::metadata(follow_symlink(&entry.path()).ok()?).ok()?
                } else {
                    return None;
                }
            }
            let file_name = entry.file_name().into_string().ok()?;
            Some((file_name, meta))
        })
        .fold(DirectoryContent::default(), |mut res, (name, meta)| {
            let size = meta.file_size();
            let modified = meta
                .modified()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let created = meta
                .created()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let is_symlink = meta.is_symlink();

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
