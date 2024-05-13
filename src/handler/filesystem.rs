use crate::config::{ImageTarget, UserConfig, PostTargetAction};
use crate::helper::LError;
use crate::notification::NotificationKind;
use crate::{clipboard, image, text};

pub fn run(config: UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), crate::helper::LError> {

    let target_location = config.generate_location()?.clone();
    if image::image_to_file(kind, target_location.clone()) == false {
        eprintln!("{}", text::message(30));
        crate::text::exit();
    }

    match config.post_target_action {
        PostTargetAction::CopyLocation => {
            copy_location(target_location)
        },
        PostTargetAction::CopyContent => {
            copy_content(target_location)
        },
        _ => {
            Err(LError::UnhandledPostTargetAction(config.post_target_action))
        }
    }
}

fn copy_location(location: String) -> Result<(), LError> {
    match clipboard::copy_text(location.clone()) {
        Ok(_) => {
            crate::notification::display(ImageTarget::Filesystem, NotificationKind::ClipboardCopy);
            Ok(())
        },
        Err(e) => {
            println!("[filesystem.copy_location] failed to copy to clipboard: {:#?}", e);
            crate::notification::error_msg(42, location);
            crate::text::exit();
        }
    }
}

fn copy_content(location: String) -> Result<(), LError> {
    match clipboard::copy_content(location.clone()) {
        Ok(_) => {
            crate::notification::display(ImageTarget::Filesystem, NotificationKind::ClipboardCopyContent);
            Ok(())
        },
        Err(e) => {
            println!("[filesystem.copy_content] failed to copy content to clipboard: {:#?}", e);
            crate::notification::error_msg(47, location);
            crate::text::exit();
        }
    }
}