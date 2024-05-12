use screenshot_rs::ScreenshotKind;
use glib::{user_special_dir, UserDirectory};
use crate::text;
use crate::helper::Error as LError;

use std::path::PathBuf;

pub const CONFIG_ACTION_DEFAULT: &str = "area";
pub const FILENAME_FORMAT_DEFAULT: &str = "%Y%m%d_%H-%H-%M.png";
pub const LOCATION_ROOT_DEFAULT: &str = "home.pictures";
pub const LOCATION_FORMAT_DEFAULT: &str = "/Screenshots/%Y-%m/";

pub fn get_default_base_path() -> Result<PathBuf, crate::helper::Error> {
    match std::env::current_dir() {
        Ok(v) => Ok(v),
        Err(e) => {
            let m: String = text::message(34).replace("%s", "std::env::current_dir()");
            Err(LError::ConfigIOError(e, m))
        }
    }
}

pub fn base_path_from_config(location_root: String) -> Result<PathBuf, crate::helper::Error> {
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
                println!("Invalid location_root {}", location_root);
            }

            Ok(default_location)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UserConfigKeyword {
    BasePath,
    FinalPath,

    ConfigContentRead,
    ConfigContentToUtf8
}
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ImageTarget {
    Filesystem,

    Twitter,
    Mastodon,
    Imgur,

    GoogleCloudStorage,
    AWS,
    XBackbone
}
impl Default for ImageTarget {
    fn default() -> Self {
        ImageTarget::Filesystem
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    #[serde(default = "get_default_action")]
    pub default_action: String,

    #[serde(default)]
    pub default_target: ImageTarget,

    #[serde(default = "get_default_filename_format")]
    pub filename_format: String,
    #[serde(default = "get_default_location_format")]
    pub location_format: String,
    #[serde(default = "get_default_location_root")]
    pub location_root: String,
    pub xbackbone_config: Option<crate::handler::xbackbone::XBackboneConfig>
}
fn get_default_action() -> String { CONFIG_ACTION_DEFAULT.to_string() }
fn get_default_filename_format() -> String { FILENAME_FORMAT_DEFAULT.to_string() }
fn get_default_location_format() -> String { LOCATION_FORMAT_DEFAULT.to_string() }
fn get_default_location_root() -> String { LOCATION_ROOT_DEFAULT.to_string() }
impl UserConfig {
    /// Create a new instance of UserConfig
    pub fn new() -> Self {
        Self {
            default_action: CONFIG_ACTION_DEFAULT.to_string(),
            default_target: ImageTarget::default(),
            filename_format: FILENAME_FORMAT_DEFAULT.to_string(),
            location_format: LOCATION_FORMAT_DEFAULT.to_string(),
            location_root: LOCATION_ROOT_DEFAULT.to_string(),
            xbackbone_config: None
        }
    }

    /// Try and get the config location
    /// Will use $home_dir/.config/rustgrab/config.json
    /// When std::env::home_dir() fails, or unwrapping it fails, then this will just return "rustgrab.config.json"
    pub fn get_config_location() -> String {
        match std::env::home_dir() {
            Some(d) => {
                let mut s = d.to_str().unwrap_or("").to_string();
                if s.ends_with("/") {
                    s.push('/');
                }
                if s.len() > 0 {
                    let config_dir = format!("{}/.config/rustgrab", s);
                    let _ = std::fs::create_dir_all(&config_dir);
                    let res = format!("{}/config.json", config_dir);
                    res
                } else {
                    "rustgrab.config.json".to_string()
                }
            },
            None => {
                "rustgrab.config.json".to_string()
            }
        }
    }

    /// Parse config from UserConfig::get_config_location()
    pub fn parse() -> Result<UserConfig, LError> {
        let location = UserConfig::get_config_location();
        UserConfig::from_location(location)
    }
    /// Parse config from location provided.
    pub fn from_location(location: String) -> Result<UserConfig, LError> {
        let content = match std::fs::read(&location) {
            Ok(v) => v,
            Err(e) => {
                return Err(LError::ConfigIOError(e, location));
            }
        };
        let content_str = match String::from_utf8(content) {
            Ok(v) => v,
            Err(e) => {
                return Err(LError::FromUtf8Error(e));
            }
        };

        match serde_json::from_str(content_str.as_str()) {
            Ok(v) => {
                Ok(v)
            },
            Err(e) => {
                Err(LError::Json(e))
            }
        }
    }

    /// This function generates the full location for a new file.
    pub fn generate_location(&self) -> Result<String, crate::helper::Error> {
        let current_date = chrono::Local::now();

        let base_safe = base_path_from_config(self.location_root.clone())?;
        let base_str = match base_safe.to_str() {
            Some(v) => v,
            None => {
                eprintln!("{}", text::message(32));
                return Err(LError::ConfigUnwrapFailure(UserConfigKeyword::BasePath));
            }
        };

        // parse and format location
        let formatted_location = self.get_location(&current_date);

        let formatted_filename = UserConfig::format_location(
            self.filename_format.clone(),
            &current_date);

        let location_res = std::path::Path::new(base_str)
            .join(format!("{}/{}", formatted_location, formatted_filename));

        match location_res.to_str() {
            Some(v) => Ok(v.to_string()),
            None => {
                eprintln!("{}", text::message(33));
                Err(LError::ConfigUnwrapFailure(UserConfigKeyword::FinalPath))
            }
        }
    }

    /// Format location so it has the date keywords and the custom keywords
    pub fn format_location(location: String, date: &chrono::DateTime<chrono::Local>) -> String {
        let mut data = format!("{}", &date.format(&location));
        let uuid = crate::helper::create_uuid();
        data = data.replace("$guid", &uuid);

        data
    }

    /// Parse the location in "location_format".
    /// Note: will not have trailing slashes.
    pub fn get_location(&self, date: &chrono::DateTime<chrono::Local>) -> String {
        // parse and format location
        let mut formatted_location = UserConfig::format_location(
            self.location_format.clone(),
            &date);
        if formatted_location.starts_with("./") {
            formatted_location.replace_range(0..2, "");
        }

        // remove any characters starting with forward slash.
        while formatted_location.starts_with("/") {
            if formatted_location.len() < 1 { // make sure that we don't break things
                break;
            }
            formatted_location.remove(0);
        }

        // remove trailing "/" in location
        while formatted_location.ends_with("/") {
            formatted_location.pop();
        }

        formatted_location
    }
}