use crate::config::{ImageTarget, UserConfig, PostTargetAction};
use crate::{clipboard, locale, LError, notification::NotificationKind};

pub fn run(config: UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), LError> {

    let target_location = config.generate_location()?.clone();
    if crate::image_to_file(kind, target_location.clone()) == false {
        eprintln!("[handler.filesystem.run] {}", locale::error(30));
        return Err(LError::ErrorCode(30));
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
            return Err(LError::ErrorCodeMsg(42, location));
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
            return Err(LError::ErrorCodeMsg(47, location));
        }
    }
}