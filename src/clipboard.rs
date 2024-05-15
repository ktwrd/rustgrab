use arboard::{Clipboard, ImageData};
use crate::{LError, ErrorSource, helper};
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;
use std::os::unix::process::ExitStatusExt;

/// Copy text to clipboard
/// text: Content to copy.
/// Err: LError::Clipboard
pub fn copy_text(text: String) -> Result<(), LError> {
    println!("[clipboard::copy_text] {}", &text);
    let mut clipboard = try_create_clipboard()?;
    match clipboard.set_text(&text) {
        Ok(_) => {
            let d = core::time::Duration::from_millis(1500);
            std::thread::sleep(d);
            Ok(())
        },
        Err(e) => Err(LError::Clipboard(e))
    }
}
/// Try and create an instance of arboard::Clipboard
/// Err: LError::Clipboard
fn try_create_clipboard() -> Result<Clipboard, LError> {
    match Clipboard::new() {
        Ok(v) => Ok(v),
        Err(e) => Err(LError::Clipboard(e))
    }
}

/// Copy the content at the location provided to the clipboard.
pub fn copy_content(location: String) -> Result<(), LError> {
    let ext = helper::get_file_extension(location.clone());
    match ext.as_str() {
        "png" => {
            copy_location_as_png(location)
        },
        _ => {
            copy_file_generic(location)
        }
    }
}

/// Copy the location provided as a PNG image.
/// Err: Can be LError::Clipboard, or an error from create_image_data
pub fn copy_location_as_png(location: String) -> Result<(), LError> {
    let data = create_image_data(location, ImageFormat::Png)?;
    let mut instance = try_create_clipboard()?;
    match instance.set_image(data) {
        Ok(_) => {
            let dur = std::time::Duration::from_secs(2);
            std::thread::sleep(dur);
            Ok(())
        },
        Err(e) => {
            Err(LError::Clipboard(e))
        }
    }
}

fn copy_file_generic(location: String) -> Result<(), LError> {
    match std::process::Command::new("xclip-copyfile")
        .arg(location)
        .status() {
        Ok(v) => {
            match v.into_raw() {
                0 => Ok(()),
                _ => {
                    Err(LError::UnhandledProcessExitStatusS(ErrorSource::CopyFileGeneric, v))
                }
            }
        },
        Err(e) => {
            Err(LError::IOS(ErrorSource::CopyFileGeneric, e))
        }
    }
}

/// Generate a static instance of ImageData from the location specified.
/// location: Full location to where the image is stored.
/// format: The image format that is passed through to DynamicImage.write_to
/// Err: Can be LError::ImageError or LError::IO
fn create_image_data(location: String, format: ImageFormat) -> Result<ImageData<'static>, LError> {
    let img = match ImageReader::open(location.clone()) {
        Ok(v) => match v.decode() {
            Ok(d) => d,
            Err(e) => {
                return Err(LError::ImageError(location.clone() , e));
            }
        },
        Err(e) => {
            return Err(LError::IO(e));
        }
    };

    let mut bytes: Vec<u8> = Vec::new();
    match img.write_to(&mut Cursor::new(&mut bytes), format) {
        Ok(_) => {
            let data = ImageData {
                width: img.width() as usize,
                height: img.height() as usize,
                bytes: std::borrow::Cow::from(bytes)
            };
            Ok(data)
        },
        Err(e) => {
            Err(LError::ImageError(location.clone(), e))
        }
    }
}