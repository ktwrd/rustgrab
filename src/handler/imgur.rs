use crate::config::ImageTarget;
use crate::{notification, locale, LError};
use imgur;
use open;
use std::fs::File;
use std::io::Read;

pub fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), LError>{
    
    let location = config.generate_location()?;
    
    if crate::image_to_file(kind, location.clone()) == false {
        eprintln!("{}", locale::error(30));
        return Ok(());
    }
    
    // Opens file for use
    let mut file = match File::open(&location) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("[handler.imgur.run] {}", locale::error(28));
            return Err(LError::ErrorCode(28));
        }
    };
    
    // Stores image in a Vector
    let mut image = Vec::new();
    if file.read_to_end(&mut image).is_err() {
        eprintln!("[handler.imgur.run] {}", locale::error(28));
        return Err(LError::ErrorCode(28));
    };

    // Creates Imgur Applications for sending to Imgur API
    let mut copy_link = String::new();
    let handle = imgur::Handle::new(String::from("37562f83e04fd66"));

    // Uploads file to Imgur API
    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("[handler.imgur.run] {}", locale::error(20));
                return Err(LError::ErrorCode(20));
            }
        },
        Err(_) => {
            eprintln!("[handler.imgur.run] {}", locale::error(17));
            return Err(LError::ErrorCode(17));
        }
    }

    // Send notification
    notification::image_sent(ImageTarget::Imgur, &copy_link, location.as_str());

    // Opens url
    match open::that(copy_link) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("[handler.imgur.run] {}", locale::error(19));
            eprintln!("[handler.imgur.run] {:#?}", e);
            notification::error(19);
            Err(LError::ErrorCode(19))
        }
    }
}