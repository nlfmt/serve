use std::path::Path;

pub fn pretty_path(path: &Path) -> String {
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
        path.to_str().unwrap().to_owned()
    }
}