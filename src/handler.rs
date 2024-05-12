use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;
use crate::{
    text,
    MessageKind,
    dialog,
    image,
    config::ImageTarget
};
use crate::helper::Error as LError;

pub mod xbackbone;
pub mod imgur;
pub mod filesystem;


pub fn run(service: ImageTarget,
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

pub fn runcfg(screenshot_kind: ScreenshotKind) {
    let location = crate::config::UserConfig::get_config_location();
    if std::path::Path::new(location.as_str()).exists() == false{
        eprintln!("{} {}", text::message(43), location);
        crate::notification::error(43);
        crate::text::exit();
    }
    println!("location: {}", location);
    match crate::config::UserConfig::parse() {
        Ok(cfg) => {
            match cfg.default_target {
                ImageTarget::Filesystem => {
                    inner_handle(cfg.default_target, crate::handler::filesystem::run(cfg, screenshot_kind));
                },
                ImageTarget::XBackbone => {
                    inner_handle(cfg.default_target, crate::handler::xbackbone::run(cfg, screenshot_kind));
                },
                ImageTarget::Imgur => {
                    inner_handle(cfg.default_target, crate::handler::imgur::run(cfg, screenshot_kind));
                },

                // handle stuff that we haven't, and let the user know.
                _ => {
                    crate::notification::error_msg(45, format!("{:#?}", cfg.default_target));
                }
            };
        },
        Err(e) => {
            crate::notification::error_body(46, format!("{:#?}", e));
            panic!("Failed to get config.\n{:#?}", e);
        }
    }
}
fn inner_handle(target: ImageTarget, res: Result<(), LError>) {
    match res {
        Ok(_) => {},
        Err(e) => {
            crate::notification::error(0);
            panic!("Failed to run {:#?}. {:#?}", target, e);
        }
    }
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