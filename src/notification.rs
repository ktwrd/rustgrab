use crate::config::ImageTarget;
use crate::locale;
use notify_rust::{Notification, Timeout};
use std::fs;
use std::{thread, time};
use yaml_rust::YamlLoader;

const APP_NAME: &str = "rustgrab";

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum NotificationKind {
    Sent,
    SendFailure,
    FileCopy,
    // Location of thing is copied
    ClipboardCopy,
    // Actual thing is copied
    ClipboardCopyContent
}

fn notification(service: ImageTarget, notification: NotificationKind) -> String {
    // Gets section of localization file for Notifications
    let mut locator = &YamlLoader::load_from_str(&locale::loader()).unwrap()[0]["Notification"];

    // Checks kind of notification and the kind of service (Twitter, Mastodon, Imgur) used
    match &notification {
        NotificationKind::Sent => {
            locator = &locator["Sent"];
            match service {
                ImageTarget::Twitter => locator = &locator["Twitter"],
                ImageTarget::Mastodon => locator = &locator["Mastodon"],
                ImageTarget::Imgur => locator = &locator["Imgur"],
                _ => unreachable!("ImageTarget {:#?} is not supported for {:#?}", service, notification)
            }
        }
        NotificationKind::SendFailure => {
            locator = &locator["Not_Sent"];
            match service {
                ImageTarget::Twitter => locator = &locator["Twitter"],
                ImageTarget::Mastodon => locator = &locator["Mastodon"],
                ImageTarget::Imgur => locator = &locator["Imgur"],
                _ => unreachable!("ImageTarget {:#?} is not supported for {:#?}", service, notification)
            }
        },
        NotificationKind::FileCopy => {
            locator = &locator["Clipboard"]["Copy"]["Filesystem"];
        },
        NotificationKind::ClipboardCopy => {
            locator = &locator["Clipboard"]["Copy"];
            match service {
                ImageTarget::Twitter => locator = &locator["Twitter"],
                ImageTarget::Mastodon => locator = &locator["Mastodon"],
                ImageTarget::Imgur => locator = &locator["Imgur"],
                ImageTarget::XBackbone => locator = &locator["XBackbone"],
                _ => unreachable!("ImageTarget {:#?} is not supported for {:#?}", service, notification)
            }
        },
        NotificationKind::ClipboardCopyContent => {
            locator = &locator["Clipboard"]["Copy"]["Content"];
        }
    }
    return format!("{}", locator.as_str().unwrap());
}
pub fn display(service: ImageTarget, notif_kind: NotificationKind) {
    let notification = match Notification::new()
        .appname(APP_NAME)
        .summary(&notification(service, notif_kind))
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", locale::error(23));
            return;
        }
    };

    thread::sleep(time::Duration::from_secs(3));
    notification.close();
}

// Sends a notification with notify-rust, when a status with an image or an image is sent/uploaded
pub fn image_sent(service: ImageTarget, text: &str, img: &str) {
    let notification = match Notification::new()
        .appname(APP_NAME)
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .icon(&img)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", locale::error(23));
            return;
        }
    };

    // Removes temporary file
    if fs::remove_file(img).is_err() {
        return;
    };

    thread::sleep(time::Duration::from_secs(3));
    notification.close();
}

// Sends a notification when a status is sent
pub fn message_sent(service: ImageTarget, text: &str) {
    let notification = match Notification::new()
        .appname(APP_NAME)
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", locale::error(23));
            return;
        }
    };

    thread::sleep(time::Duration::from_secs(3));
    notification.close();
}

// Sends a notification when a status update didn't go through
pub fn not_sent(service: ImageTarget) {
    if Notification::new()
        .appname(APP_NAME)
        .summary(&notification(service, NotificationKind::SendFailure))
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", locale::error(23));
        return;
    };
}

// Sends a notification with the error message as the body
pub fn error(code: usize) {
    if Notification::new()
        .appname(APP_NAME)
        .summary(&locale::error(code))
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", locale::error(23));
        return;
    };
}
pub fn error_msg(code: usize, msg: String) {
    let summary = locale::error(code)
        .replace("%s", msg.as_str());
    if Notification::new()
        .appname(APP_NAME)
        .summary(&summary)
        .timeout(Timeout::Milliseconds(5000))
        .show()
        .is_err()
    {
        eprintln!("{}", locale::error(23));
        return;
    };
}
pub fn error_body(code: usize, body: String) {
    if Notification::new()
        .appname(APP_NAME)
        .summary(&locale::error(code))
        .body(&body)
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", locale::error(23));
        return;
    };
}

// Useful when Terminal is not available (if launched without one)
#[allow(dead_code)]
pub fn debug(error: String) {
    if Notification::new()
        .appname(APP_NAME)
        .summary(&error.to_string())
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", locale::error(23));
        return;
    };
}
