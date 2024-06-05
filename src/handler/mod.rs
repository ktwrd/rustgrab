use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;
use crate::{
    config::{cfg_init_or_die, ImageTarget, TargetAction, UserConfig}, locale, LError
};

use self::post_upload_action::PostUploadActionHandler;

pub mod imgur;
pub mod filesystem;
pub mod xbackbone;
pub mod post_upload_action;
pub mod gcs;

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
    run_screenshot(cfg.clone(), cfg.default_target, cfg.default_screenshot_type.to()).await
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
        ImageTarget::GoogleCloudStorage => {
            crate::handler::gcs::screenshot(cfg, kind).await
        }

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
    let cfg_location = cfg_init_or_die();
    println!("config location: {}", cfg_location);
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

/// Generate & Parse Config, then call run_default_upload.
pub async fn run_default_upload_cfg(location: String) {
    let cfg_location = cfg_init_or_die();
    println!("config location: {}", cfg_location);

    match crate::config::UserConfig::parse() {
        Ok(cfg) => {
            run_default_upload(cfg, location).await;
        },
        Err(e) => {
            crate::msgbox::error(46);
            panic!("Failed to get config.\n{:#?}", e);
        }
    }
}

/// Upload file to default target
pub async fn run_default_upload(cfg: UserConfig, location: String) {
    let t = cfg.default_target.clone();
    run_upload(cfg, t, location).await;
}

/// Upload file to target specified.
pub async fn run_upload(cfg: UserConfig, target: ImageTarget, location: String) {
    let c = cfg.clone();
    let h = match target {
        ImageTarget::Filesystem => {
            Err(LError::ErrorCodeMsg(49, format!("{:#?}", TargetAction::Upload)))
        },
        ImageTarget::XBackbone => {
            crate::handler::xbackbone::upload(cfg, location)
        },
        ImageTarget::Imgur => {
            crate::handler::imgur::upload(cfg, location).await
        },
        ImageTarget::GoogleCloudStorage => {
            crate::handler::gcs::upload(cfg, location).await
        }

        // handle stuff that we haven't, and let the user know.
        _ => {
            Err(LError::ErrorCodeMsg(45, format!("{:#?}", target)))
        }
    };
    inner_handle(target, h, c);
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
                        handle_error_fatal(target, e);
                    },
                    _ => {}
                },
                _ => println!("{:#?} not handled for a post-upload action", v)
            };
        },
        Err(e) => {
            handle_error_fatal(target, e);
        }
    }
}
const IGNORE_CODES: &'static [&'static usize] = &[&30];
fn do_ignore_code(code: usize) -> bool {
    for x in IGNORE_CODES.into_iter() {
        if x == &&code {
            return true;
        }
    }
    return false;
}
/// Handle fatal errors for inner handling.
/// Shows message box then panics.
fn handle_error_fatal(target: ImageTarget, e: LError) {
    let mut show_extended = true;
    let content = match &e {
        LError::ErrorCodeMsg(code, val) => {
            if do_ignore_code(*code) {
                println!("[handler::handle_error_fatal] ignored code {}", code);
                return;
            }
            show_extended = false;
            locale::error_msg(*code, val.clone())
        },
        LError::ErrorCode(code) => {
            if do_ignore_code(*code) {
                println!("[handler::handle_error_fatal] ignored code {}", code);
                return;
            }
            show_extended = false;
            locale::error(*code)
        },
        _ => format!("{:#?}", e)
    };
    let mut msgbox_text = format!("Failed to handle target {:#?}\n\n{:#?}", target, content);
    if show_extended == false {
        msgbox_text = content;
    }
    crate::msgbox::error_custom(
        msgbox_text,
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