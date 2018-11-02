use error;
use image;
use notification;
use std::process::Command;
use ServiceKind;

pub fn image(status: String) {
    let service = ServiceKind::Mastodon;

    let tmp = image::temp_dir();
    let temp = tmp.to_str().unwrap().clone();

    // Calls the "toot" Python app and sends a status with an image
    let toot = match Command::new("toot")
        .args(&["post", "-m", &temp, &status])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(service);
            error::exit()
        }
    };

    // If toot gives the error code 2, then the status was not sent
    if toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::image_sent(service, &status, temp);
    }
}

pub fn toot(status: String) {
    let service = ServiceKind::Mastodon;

    // Calls the "toot" Python app and send a status
    let toot = match Command::new("toot").args(&["post", &status]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(service);
            error::exit()
        }
    };

    // If toot gives the error code 2, then the status was not sent
    if toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::message_sent(service, &status);
    }
}

pub fn auth() {
    // Calls the "toot" Python app and asks to login using the browser
    if Command::new("toot").arg("login").status().is_err() {
        eprintln!("{}", error::message(6));
        error::exit();
    };
}
