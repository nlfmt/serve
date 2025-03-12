use std::{
    fmt::Display, fs, io, os::windows::fs::MetadataExt, path::{Component, Path, PathBuf}, time::UNIX_EPOCH
};

use crate::models::{DirectoryContent, FileInfo, FolderInfo};

pub fn pretty_path(path: &Path) -> impl Display {
    #[cfg(target_os = "windows")]
    {
        let mut path_str = path.to_str().unwrap().to_owned();
        if let Some(p) = path_str.strip_prefix(r"\\?\") {
            path_str = p.to_string()
        }
        path_str
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.display()
    }
}

fn follow_symlink(link: &Path) -> io::Result<PathBuf> {
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
                    return None
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

fn path_contains_symlink(path: &Path) -> io::Result<bool> {
    let mut current = PathBuf::new();

    for component in path.components() {
        current.push(component);
        
        // Check if this component is a symlink
        if let Ok(metadata) = fs::symlink_metadata(&current) {
            if metadata.file_type().is_symlink() {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

pub fn parse_relative_path(root: &Path, path: &str, allow_symlinks: bool) -> Option<PathBuf> {
    let path = Path::new(path);
    let components = path.components();
    let mut stack = Vec::new();

    for c in components {
        match c {
            Component::Prefix(_) => {
                return None;
            }
            Component::CurDir | Component::RootDir => {}
            Component::ParentDir => {
                if stack.pop().is_none() {
                    return None;
                }
            }
            Component::Normal(part) => {
                stack.push(part.to_str()?);
            }
        }
    }

    let joined = root.join(stack.join("/"));
    
    match allow_symlinks {
        true => Some(joined),
        false => if path_contains_symlink(&joined).ok()? {
            None
        } else {
            Some(joined)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Path::new("C:/test");
        assert!(parse_relative_path(&root, "/abc/def", false).is_some());
        assert!(parse_relative_path(&root, "./abc/def", false).is_some());
        assert!(parse_relative_path(&root, r"\abc\def", false).is_some());
        assert!(parse_relative_path(&root, r".\abc\def", false).is_some());
        assert!(parse_relative_path(&root, "/abc/../def", false).is_some());
        assert!(parse_relative_path(&root, "abc/../def", false).is_some());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../", false).is_some());

        // prefixes should not be allowed
        assert!(parse_relative_path(&root, "C:/abc/def", false).is_none());
        assert!(parse_relative_path(&root, "D:/abc/def", false).is_none());

        // paths that move beyond the root dir at any point should not be allowed
        assert!(parse_relative_path(&root, "abc/../../def", false).is_none());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../../", false).is_none());
    }
}
