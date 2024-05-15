use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;
use crate::{
    config::{ImageTarget, UserConfig}, locale, LError, MessageKind
};
use std::process;

use self::post_upload_action::PostUploadActionHandler;

pub mod imgur;
pub mod filesystem;
pub mod xbackbone;
pub mod post_upload_action;

#[allow(unused, unreachable_code)]
pub fn run(service: ImageTarget,
            create_file_when_not_set: bool,
            target_file: Option<String>,
            screenshot_kind: Option<ScreenshotKind>)
    -> Result<(), LError>
{
    let mut has_screenshot = match target_file {
        Some(_) => true,
        None => false
    };
    let location = match target_file {
        Some(v) => v,
        None => {
            let i = crate::config::UserConfig::new();
            match i.generate_location() {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };
    let kind = match screenshot_kind {
        Some(v) => v,
        None => {
            println!("[handler.run] No screenshot kind was provided, defaulting to Area");
            ScreenshotKind::Area
        }
    };

    let mut success = false;
    let mut image_called = false;
    if has_screenshot {
        success = crate::image_to_file(kind, location);
        image_called = true;
    } else {
        if create_file_when_not_set {
            success = crate::image_to_file(kind, location);
            image_called = true;
        }
    }

    if success == false && image_called == true {
        eprintln!("[handler.run] failed to create screenshot. assuming it was probably aborted by user.");
        return Ok(());
    }

    let mut message_kind = MessageKind::Text;
    if has_screenshot || (image_called && success) {
        message_kind = MessageKind::Image;
    }

    todo!("use the dialog package for generating the old method of a dialog for tweet/toot");
    //crate::dialog::dialog(service, message_kind);
    Ok(())
}

pub async fn runcfg(screenshot_kind: ScreenshotKind) {
    let location = crate::config::UserConfig::get_config_location();
    if std::path::Path::new(location.as_str()).exists() == false{
        eprintln!("{} {}", locale::error(43), location);
        crate::notification::error(43);
        process::exit(1);
    }
    println!("location: {}", location);
    match crate::config::UserConfig::parse() {
        Ok(cfg) => {
            let c = cfg.clone();
            match cfg.default_target {
                ImageTarget::Filesystem => {
                    inner_handle(cfg.default_target, crate::handler::filesystem::run(cfg, screenshot_kind), c);
                },
                ImageTarget::XBackbone => {
                    inner_handle(cfg.default_target, crate::handler::xbackbone::run(cfg, screenshot_kind), c);
                },
                ImageTarget::Imgur => {
                    let t = cfg.default_target.clone();
                    let r = crate::handler::imgur::run(cfg, screenshot_kind).await;
                    inner_handle(t, r, c);
                },

                // handle stuff that we haven't, and let the user know.
                _ => {
                    crate::notification::error_msg(45, format!("{:#?}", cfg.default_target));
                }
            };
        },
        Err(e) => {
            crate::msgbox::error(46);
            panic!("Failed to get config.\n{:#?}", e);
        }
    }
}
fn inner_handle(target: ImageTarget, res: Result<TargetResultData, LError>, cfg: UserConfig) {
    match res {
        Ok(v) => {
            let x = PostUploadActionHandler {
                config: cfg,
                target: target.clone()
            };
            match v {
                TargetResultData::Upload(u) => match x.run(u) {
                    Err(e) => {
                        handle_lerror_fatal(target, e);
                    },
                    _ => {}
                },
                _ => println!("{:#?} not handled for a post-upload action", v)
            };
        },
        Err(e) => {
            handle_lerror_fatal(target, e);
        }
    }
}
/// Handle fatal errors for inner handling.
fn handle_lerror_fatal(target: ImageTarget, e: LError) {
    crate::msgbox::error_custom(
        format!("Failed to handle target {:#?}\n\n{:#?}", target, e),
        format!("Failed to handle target"));
    panic!("Failed to run {:#?}. {:#?}", target, e);
}

/// OK Result for a handler target.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum TargetResultData {
    Upload(TargetResultUploadData),
    Filesystem(String)
}
/// Result data from a target in handler. Only used when the target uploads to something.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TargetResultUploadData {
    pub fs_location: String,
    pub url: String
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