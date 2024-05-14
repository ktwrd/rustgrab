use std::path::PathBuf;
use glib::{user_special_dir, UserDirectory};
use crate::{LError, locale};

pub fn create_uuid() -> String {
    let value = uuid::Uuid::new_v4();
    value.to_string()
}

pub fn try_get_file_content(location: String) -> Result<Vec<u8>, LError> {
    match std::fs::read(location.clone()) {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(LError::IO(e))
        }
    }
}
pub fn get_default_base_path() -> Result<PathBuf, LError> {
    match std::env::current_dir() {
        Ok(v) => Ok(v),
        Err(e) => {
            let m: String = locale::error(34).replace("%s", "std::env::current_dir()");
            Err(crate::helper::LError::ConfigIOError(e, m))
        }
    }
}
pub fn base_path_from_config(location_root: String) -> Result<PathBuf, crate::helper::LError> {
    let default_location = get_default_base_path()?;

    let base: Option<PathBuf> = match location_root.as_str() {
        "home" => Some(glib::home_dir()),
        "home.desktop" => user_special_dir(UserDirectory::Desktop),
        "home.pictures" => user_special_dir(UserDirectory::Pictures),
        "home.documents" => user_special_dir(UserDirectory::Documents),
        _ => Some(default_location.clone())
    };
    match base {
        Some(v) => Ok(v),
        None => {
            if location_root.len() > 0 {
                println!("[WARN] Invalid location_root {}", location_root);
            }

            Ok(default_location)
        }
    }
}
pub fn get_file_extension(location: String) -> String {
    std::path::Path::new(location.as_str())
        .extension()
        .and_then(std::ffi::OsStr::to_str).unwrap_or("").to_string()
}