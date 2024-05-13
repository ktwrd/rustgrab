use crate::config::ImageTarget;
use crate::notification::NotificationKind;
use crate::{clipboard, image, text};

pub fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), crate::helper::LError> {

    let target_location = config.generate_location()?.clone();
    let filename = &target_location.split('/').into_iter().last().unwrap();
    let filename_str = filename.to_string();
    if image::image_to_file(kind, target_location.clone()) == false {
        eprintln!("{}", text::message(30));
        crate::text::exit();
    }

    todo!();
}

fn copy_location(location: String) {
    match clipboard::copy_text(location.clone()) {
        Ok(_) => {
            crate::notification::display(ImageTarget::Filesystem, NotificationKind::ClipboardCopy);
        },
        Err(e) => {
            println!("[filesystem.copy_location] failed to copy to clipboard: {:#?}", e);
            crate::notification::error_msg(42, location);
            crate::text::exit();
        }
    }
}

fn copy_content(_location: String) {
    todo!();
}