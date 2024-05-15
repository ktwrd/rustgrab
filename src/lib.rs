pub mod handler;
pub mod locale;
pub mod helper;
pub mod config;
pub mod clipboard;
pub mod notification;
pub mod msgbox;

use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub enum LError {
    ConfigPathBaseFailure(String),
    // UserConfig failed to unwrap something.
    ConfigUnwrapFailure(crate::config::UserConfigKeyword),
    ConfigIOError(std::io::Error, String),
    
    FromUtf8Error(std::string::FromUtf8Error),
    Json(serde_json::Error),
    IO(std::io::Error),
    IOS(ErrorSource, std::io::Error),
    Clipboard(arboard::Error),
    GetHomeError(homedir::GetHomeError),
    Imgur(imgurs::Error),
    ImgurFailure(imgurs::ImageInfo),

    HomeDirectoryNotSet,
    
    ErrorCode(usize),
    ErrorCodeMsg(usize, String),
    ErrorCodeE(usize, Box<dyn std::error::Error>),

    UnhandledProcessExitStatus(std::process::ExitStatus),
    UnhandledProcessExitStatusS(ErrorSource, std::process::ExitStatus),

    UnhandledPostTargetAction(crate::config::PostTargetAction),

    ImageError(String, image::error::ImageError),

    UnhandledFileExtension(String)
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ErrorSource {
    Unknown,
    TryGetFileContent,
    CopyFileGeneric
}
impl Default for ErrorSource {
    fn default() -> Self {
        ErrorSource::Unknown
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum MessageKind {
    Image,
    Text,
}

use screenshot_rs::{
    screenshot_area,
    screenshot_window,
    screenshot_full,
    ScreenshotKind
};
/// Take a screenshot and write it to the location provided.
/// returns: if the file exists or not (pretty much if it's successful)
pub fn image_to_file(kind: ScreenshotKind, location: String) -> bool {
    // Matches the kind of screenshot to functions in the screenshot-rs library
    match kind {
        ScreenshotKind::Area => screenshot_area(location.clone(), true),
        ScreenshotKind::Window => screenshot_window(location.clone()),
        ScreenshotKind::Full => screenshot_full(location.clone()),
    };
    std::path::Path::new(location.as_str()).exists()
}