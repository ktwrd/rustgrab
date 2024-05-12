use crate::config::ImageTarget;
use crate::{image, notification, text};
use imgur;
use open;
use std::fs::File;
use std::io::Read;
use crate::helper::Error as LError;

pub fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), LError>{
    
    let location = config.generate_location()?;
    
    if crate::image::image_to_file(kind, location.clone()) == false {
        eprintln!("{}", text::message(30));
        return Ok(());
    }
    
    // Opens file for use
    let mut file = match File::open(&location) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(28));
            notification::error(28);
            text::exit()
        }
    };
    
    // Stores image in a Vector
    let mut image = Vec::new();
    if file.read_to_end(&mut image).is_err() {
        eprintln!("{}", text::message(28));
        notification::error(28);
        text::exit();
    };

    // Creates Imgur Applications for sending to Imgur API
    let mut copy_link = String::new();
    let handle = imgur::Handle::new(String::from("37562f83e04fd66"));

    // Uploads file to Imgur API
    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("{}", text::message(20));
                notification::error(20);
                text::exit()
            }
        },
        Err(_) => {
            eprintln!("{}", text::message(17));
            notification::error(17);
            text::exit()
        }
    }

    // Send notification
    notification::image_sent(ImageTarget::Imgur, &copy_link, location.as_str());

    // Opens url
    match open::that(copy_link) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", text::message(19));
            eprintln!("{:#?}", e);
            notification::error(19);
            Err(LError::ErrorCode(19))
        }
    }
}