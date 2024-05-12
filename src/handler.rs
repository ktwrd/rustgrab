use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;
use crate::{
    MessageKind,
    dialog,
    image,
    ServiceKind
};


pub fn run(service: ServiceKind,
                    create_file_when_none: bool,
                    target_file: Option<String>,
                    sceenshot_kind: Option<ScreenshotKind>)
{
    let mut has_screenshot = match target_file {
        Some(_) => true,
        None => false
    };

    match target_file
    {
        Some(f) => image::file(f),
        None => {
            if create_file_when_none {
                let k = sceenshot_kind.unwrap_or(ScreenshotKind::Area);
                image::image(k);
                has_screenshot = true;
            }
        }
    };

    let message_kind = match has_screenshot {
        true => MessageKind::Image,
        false => MessageKind::Text
    };

    dialog::dialog(service, message_kind);
}

pub fn arg_to_kind(matches: &ArgMatches) -> Option<ScreenshotKind>
{
    match matches.subcommand_name() {
        Some("area") => Some(ScreenshotKind::Area),
        Some("window") => Some(ScreenshotKind::Window),
        Some("full") => Some(ScreenshotKind::Full),
        _ => None
    }
}