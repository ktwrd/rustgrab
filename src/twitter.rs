use crate::{image, notification, text, config::ImageTarget};
use std::process::Command;

pub fn image(status: String) {
    let service = ImageTarget::Twitter;

    let tmp = image::temp_dir();
    let temp = tmp.to_str().unwrap();

    // Calls the "t" Ruby app and sends a staus with an image
    let t = match Command::new("t")
        .args(&["update", &status, "-f", &temp])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", locale::error(5));
            notification::not_sent(service);
            text::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if t.code() == Some(1) {
        eprintln!("{}", locale::error(22));
        notification::not_sent(service);
        text::exit();
    } else {
        notification::image_sent(service, &status, temp);
    }
}

pub fn tweet(status: String) {
    let service = ImageTarget::Twitter;

    // Calls the "t" Ruby app and sends a staus
    let t = match Command::new("t").args(&["update", &status]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", locale::error(5));
            notification::not_sent(service);
            text::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if t.code() == Some(1) {
        eprintln!("{}", locale::error(22));
        notification::not_sent(service);
        text::exit();
    } else {
        notification::message_sent(service, &status);
    }
}

pub fn auth() {
    // Calls the "t" Ruby app and asks the user to login
    if Command::new("t").arg("authorize").status().is_err() {
        eprintln!("{}", locale::error(5));
        text::exit();
    };
}
