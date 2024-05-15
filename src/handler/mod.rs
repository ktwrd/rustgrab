use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;
use crate::{
    config::{cfg_init_or_die, ImageTarget, TargetAction, UserConfig}, LError
};

use self::post_upload_action::PostUploadActionHandler;

pub mod imgur;
pub mod filesystem;
pub mod xbackbone;
pub mod post_upload_action;

/// Generate & Parse Config, Then call `run_default`
pub async fn run_default_cfg() {
    let _ = crate::config::cfg_init_or_die();
    match crate::config::UserConfig::parse() {
        Ok(cfg) => {
            run_default(cfg).await;
        },
        Err(e) => {
            crate::msgbox::error(46);
            panic!("Failed to get config.\n{:#?}", e);
        }
    };
}

/// Run the default action & target specified in the config file.
pub async fn run_default(cfg: UserConfig) {
    let default_action = cfg.default_action.clone();
    match default_action.clone() {
        TargetAction::Screenshot => run_default_screenshot(cfg).await,
        _ => {
            crate::msgbox::error_msg(45, format!("{:#?}", default_action));
            panic!("Unhandled default action {:#?}", default_action);
        }
    }
}
/// Take a screenshot with the default kind & target defined in the config file.
pub async fn run_default_screenshot(cfg: UserConfig) {
    let screenshot_type = match crate::config::parse_screenshot_action(cfg.default_screenshot_type.clone()) {
        Ok(v) => v,
        Err(_) => {
            crate::msgbox::error_msg(48, cfg.default_screenshot_type);
            std::process::exit(1);
        }
    };
    run_screenshot(cfg.clone(), cfg.default_target, screenshot_type).await
}

/// Take a screenshot while providing the Target and kind.
pub async fn run_screenshot(cfg: UserConfig, target: ImageTarget, kind: ScreenshotKind) {
    let c = cfg.clone();
    let t = cfg.default_target.clone();
    let h = match cfg.default_target {
        ImageTarget::Filesystem => {
            crate::handler::filesystem::run(cfg, kind)
        },
        ImageTarget::XBackbone => {
            crate::handler::xbackbone::run(cfg, kind)
        },
        ImageTarget::Imgur => {
            crate::handler::imgur::run(cfg, kind).await
        },

        // handle stuff that we haven't, and let the user know.
        _ => {
            Err(LError::ErrorCodeMsg(45, format!("{:#?}", target)))
        }
    };
    inner_handle(t, h, c);
}

/// Generate & Parse Config, then call run_screenshot
/// default_target will be used when target is None.
pub async fn run_screenshot_cfg(target: Option<ImageTarget>, screenshot_kind: ScreenshotKind) {
    let location = cfg_init_or_die();
    println!("location: {}", location);
    match crate::config::UserConfig::parse() {
        Ok(cfg) => {
            let c = cfg.clone();
            let t = match target {
                Some(v) => v,
                None => cfg.default_target.clone()
            };
            run_screenshot(c, t, screenshot_kind).await;
        },
        Err(e) => {
            crate::msgbox::error(46);
            panic!("Failed to get config.\n{:#?}", e);
        }
    }
}

pub async fn run_upload(cfg: UserConfig, location: String) {
    let c = cfg.clone();
    let t = cfg.default_target.clone();
    let h = match cfg.default_target {
        ImageTarget::Filesystem => {
            Err(LError::ErrorCodeMsg(49, format!("{:#?}", TargetAction::Upload)))
        },
        ImageTarget::XBackbone => {
            crate::handler::xbackbone::upload(cfg, location)
        },
        ImageTarget::Imgur => {
            crate::handler::imgur::upload(cfg, location).await
        },

        // handle stuff that we haven't, and let the user know.
        _ => {
            Err(LError::ErrorCodeMsg(45, format!("{:#?}", t)))
        }
    };
    inner_handle(t, h, c);
}

/// Handle the result of a handle (i.e; crate::handler::imgur::run)
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
/// Shows message box then panics.
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