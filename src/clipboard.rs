use std::path::PathBuf;
use arboard::{Clipboard, ImageData};
use crate::helper::{ErrorSource, try_get_file_content};
use crate::helper::LError;
use std::fs::File;

pub fn copy_text(text: String) -> Result<(), LError> {
    let mut clipboard = try_create_clipboard()?;
    match clipboard.set_text(&text) {
        Ok(_) => {
            println!("clipboard_copy_text: {}", text);
            let dur = std::time::Duration::from_secs(2);
            std::thread::sleep(dur);
            Ok(())
        },
        Err(e) => Err(LError::Clipboard(e))
    }
}
fn try_create_clipboard() -> Result<Clipboard, LError> {
    match Clipboard::new() {
        Ok(v) => Ok(v),
        Err(e) => Err(LError::Clipboard(e))
    }
}
pub fn clipboard_copy_content(location: String) -> Result<(), LError> {
    let mut clipboard = try_create_clipboard()?;
    let image = create_image_data(location)?;
    // clipboard.set_image(image);
    Ok(())
}
pub fn create_image_data(location: String) -> Result<(), LError> {
    if &location.ends_with(".png") == &false {
        return Err(LError::UnhandledFileExtension("png".to_string()));
    }

    let content = match try_get_file_content(location.clone()) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
            /*eprintln!("{} {}", text::message(19), target_location.clone());
            eprintln!("{:#?}", e);
            crate::notification::error_body(19, format!("{}", e));
            crate::text::exit();*/
        }
    };

    let mut decoder = png::Decoder::new(match File::open(location.clone())
    {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::IO(e));
        }
    });
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = match decoder.read_info() {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::PngDecodingError(location.clone(), e));
        }
    };
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = match reader.next_frame(&mut img_data) {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::PngDecodingError(location, e));
        }
    };
    todo!();
}