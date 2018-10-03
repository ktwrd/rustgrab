use error;
use notification;
use open;
use save;
use screenshot_rs;
use std::path::PathBuf;
use std::{env, fs};

pub fn image(args: usize) {
    // tmp gets the temporary directory of the system
    let tmp = temp_dir();

    // makes a string
    let temp = tmp.to_str().unwrap().to_string();

    if args == 0 {
        screenshot_rs::screenshot_area(temp);
    } else if args == 1 {
        screenshot_rs::screenshot_window(temp);
    } else {
        screenshot_rs::screenshot_full(temp);
    }

    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        notification::error(30);
        error::fatal();
    }

    save::save();
}

pub fn delete_temp() {
    match fs::remove_file(temp_dir()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(0));
            notification::error(0);
        }
    }
}

pub fn open_temp() {
    match open::that(temp_dir()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(19));
            notification::error(19);
            return;
        }
    };
}

pub fn temp_dir() -> PathBuf {
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    return tmp;
}
