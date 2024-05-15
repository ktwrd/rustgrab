use screenshot_rs::ScreenshotKind;
use serde::{Deserialize, Serialize};
use crate::locale;
use crate::LError as LError;
use crate::helper;

pub const DEFAULT_SCREENSHOT_ACTION: &str = "area";
pub const FILENAME_FORMAT_DEFAULT: &str = "%Y%m%d_%H-%H-%M.png";
pub const LOCATION_ROOT_DEFAULT: &str = "home.pictures";
pub const LOCATION_FORMAT_DEFAULT: &str = "/Screenshots/%Y-%m/";

#[derive(Debug, Clone, Copy)]
pub enum UserConfigKeyword {
    BasePath,
    FinalPath
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ImageTarget {
    /// src/handler/filesystem.rs
    Filesystem,

    /// todo: exists in broken form. see src/twitter.rs
    Twitter,
    /// todo: exists in broken form. see src/mastodon.rs
    Mastodon,
    /// todo: not tested. src/handler/imgur.rs
    Imgur,

    /// todo
    GoogleCloudStorage,
    /// todo
    AWS,
    /// src/handler/xbackbone.rs
    XBackbone
}
impl Default for ImageTarget {
    fn default() -> Self {
        ImageTarget::Filesystem
    }
}
/// Action that happens after the Image target is successful.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum PostTargetAction {
    CopyLocation,
    CopyContent,
    ShortenLocation
}
impl Default for PostTargetAction {
    fn default() -> Self {
        PostTargetAction::CopyLocation
    }
}
/// Action that is ran after the file has been uploaded.
/// This will be ignored when ImageTarget::FileSystem target is used.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum PostUploadAction {
    CopyLink,
    ShortenLink
}
impl Default for PostUploadAction {
    fn default() -> Self {
        PostUploadAction::CopyLink
    }
}
/// What action should be taken.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TargetAction {
    Screenshot,
    Upload
}
impl Default for TargetAction {
    fn default() -> Self {
        TargetAction::Screenshot
    }
}
/// Try and parse UserConfig.default_screenshot_type
pub fn parse_screenshot_action(action: String) -> Result<ScreenshotKind, LError>
{
    let mut m = action.to_uppercase();
    m = m.trim().to_string();
    match m.as_str() {
        "AREA" => Ok(ScreenshotKind::Area),
        "A" => Ok(ScreenshotKind::Area),
        "WINDOW" => Ok(ScreenshotKind::Window),
        "W" => Ok(ScreenshotKind::Window),
        "FULL" => Ok(ScreenshotKind::Full),
        "F" => Ok(ScreenshotKind::Full),
        _ => {
            Err(LError::ScreenshotKindParseFailure(m.clone()))
        }
    }
}
pub fn kind_to_string(kind: ScreenshotKind) -> String
{
    match kind {
        ScreenshotKind::Area => "Area",
        ScreenshotKind::Full => "Full",
        ScreenshotKind::Window => "Window"
    }.to_string()
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserConfig {
    /// Default action that is used when the "default" sub-command is used.
    #[serde(default)]
    pub default_action: TargetAction,
    /// Default action that is used when no screenshot kind is provided via the CLI.
    #[serde(default = "get_default_screenshot_type")]
    pub default_screenshot_type: String,

    /// Default image/upload target when taking a screenshot or uploading a file.
    #[serde(default)]
    pub default_target: ImageTarget,

    /// Action that happens after a screenshot is taken or before a file is uploaded.
    #[serde(default)]
    pub post_target_action: PostTargetAction,
    /// Action that is taken after a screenshot/file is uploaded.
    #[serde(default)]
    pub post_upload_action: PostUploadAction,

    #[serde(default = "get_default_filename_format")]
    pub filename_format: String,
    #[serde(default = "get_default_location_format")]
    pub location_format: String,
    #[serde(default = "get_default_location_root")]
    pub location_root: String,
    pub xbackbone_config: Option<crate::handler::xbackbone::XBackboneConfig>,
    pub imgur_config: Option<crate::handler::imgur::ImgurConfig>
}
fn get_default_screenshot_type() -> String { kind_to_string(ScreenshotKind::Area) }
fn get_default_filename_format() -> String { FILENAME_FORMAT_DEFAULT.to_string() }
fn get_default_location_format() -> String { LOCATION_FORMAT_DEFAULT.to_string() }
fn get_default_location_root() -> String { LOCATION_ROOT_DEFAULT.to_string() }
impl UserConfig {
    /// Create a new instance of UserConfig
    pub fn new() -> Self {
        Self {
            default_action: TargetAction::default(),
            default_screenshot_type: get_default_screenshot_type(),
            default_target: ImageTarget::default(),
            post_target_action: PostTargetAction::default(),
            post_upload_action: PostUploadAction::default(),
            filename_format: FILENAME_FORMAT_DEFAULT.to_string(),
            location_format: LOCATION_FORMAT_DEFAULT.to_string(),
            location_root: LOCATION_ROOT_DEFAULT.to_string(),
            xbackbone_config: None,
            imgur_config: None
        }
    }

    /// Get the home directory.
    /// Returns OK with an empty string when PathBuf::to_str() returns None
    /// Returns Err when homedir::get_my_home() OK is None, or is Err
    fn get_home_dir() -> Result<String, LError> {
        match homedir::get_my_home() {
            Ok(v) => {
                match v {
                    Some(x) => Ok(x.to_str().unwrap_or("").to_string()),
                    None => Err(LError::HomeDirectoryNotSet)
                }
            },
            Err(e) => Err(LError::GetHomeError(e))
        }
    }

    /// Try and get the config location
    /// Will use $home_dir/.config/rustgrab/config.json
    /// When get_home_dir() fails, or unwrapping it fails, then this will just return "rustgrab.config.json"
    pub fn get_config_location() -> String {
        match UserConfig::get_home_dir() {
            Ok(mut s) => {
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
            Err(e) => {
                eprintln!("[config.UserConfig::get_config_location] [WARN] failed to get homedir {:#?}", e);
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
    pub fn generate_location(&self) -> Result<String, LError> {
        let current_date = chrono::Local::now();

        let base_safe = helper::base_path_from_config(self.location_root.clone())?;
        let base_str = match base_safe.to_str() {
            Some(v) => v,
            None => {
                eprintln!("{}", locale::error(32));
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
                eprintln!("{}", locale::error(33));
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
pub(crate) fn cfg_init() -> Result<String, (usize, String)> {
    let location = crate::config::UserConfig::get_config_location();
    if std::path::Path::new(location.as_str()).exists() == false{
        eprintln!("{} {}", locale::error(43), location);
        return Err((43, location));
    }
    Ok(location)
}
pub(crate) fn cfg_init_or_die() -> String {
    match cfg_init() {
        Ok(v) => v,
        Err((ec, es)) => {
            crate::msgbox::error_msg(ec, es);
            std::process::exit(1);
        }
    }
}