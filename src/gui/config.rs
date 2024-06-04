use std::sync::RwLock;
use fltk::{prelude::*, *};
use lazy_static::lazy_static;
use crate::config::{ImageTarget, LScreenshotType, PostTargetAction, PostUploadAction, TargetAction, UserConfig};
use crate::{fltk_choice_set_lazystatic, fltk_set_lazystatic, fltk_set_lazystatic_option, fltk_set_lazystatic_option_withvalue, GUIChoice};
use crate::gui::config_ui::ConfigUserInterface;
use crate::handler::gcs::GCSConfig;
use crate::handler::xbackbone::XBackboneConfig;

lazy_static!{
    static ref CURRENT_CONFIG: RwLock<UserConfig> = RwLock::new(UserConfig::new());
    static ref CURRENT_UI: RwLock<ConfigUserInterface> = RwLock::new(ConfigUserInterface::make_window());
}
static mut CALLED_ALREADY: bool = false;
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum Status {
    Update,
    Quit,
}
/// Run ConfigUserInterface. Can only be called once, otherwise will panic.
pub fn run() {
    unsafe {
        if CALLED_ALREADY {
            panic!("gui::config::run() has been called already!");
        }
        CALLED_ALREADY = true;
    }

    init();
}
/// Window icon for ConfigUserInterface as PNG.
pub const WINDOW_ICON: &[u8] = include_bytes!("config_ui.png");
fn init() {
    let app = app::App::default();
    let (ct, wt) = crate::gui::get_theme();
    ct.apply();
    wt.apply();
    let (send_action, receive_action) = app::channel::<Status>();
    let cfg_data = match std::path::Path::new(&UserConfig::get_config_location()).exists() {
        true => UserConfig::parse().expect("Failed to read config"),
        false => UserConfig::new()
    };
    if let Ok(mut c) = CURRENT_CONFIG.write() {
        *c = cfg_data;
    }

    if let Ok(mut ui) = CURRENT_UI.write() {
        ui.win.make_resizable(false);
        ui.win.show();
        ui.win.handle(move |w, ev| match ev {
            enums::Event::Resize => {
                if w.width() > 640 || w.height() > 260 {
                    w.set_size(640, 260);
                }
                true
            },
            _ => false
        });
        match image::PngImage::from_data(WINDOW_ICON) {
            Ok(img) => {
                ui.win.set_icon(Some(img));
            },
            Err(e) => {
                eprintln!("[gui::config::init] failed to set window icon {:#?}", e);
            }
        };
        ui.tabs.emit(send_action, Status::Update);
        ui.btn_save.set_callback(move |_| {
            btn_save_click();
        });
        ui.btn_cancel.set_callback(move |_| {
            println!("Operation cancelled by user.");
            app.quit();
        });
    }

    tab_general_init();
    tab_defaults_init();
    tab_xbackbone_init();
    tab_gcs_init();

    while app.wait() {
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Status::Quit => {
                    app.quit();
                },
                _ => {}
            }
        }
    }
}

fn btn_save_click() {
    panic!("[btn_save_click] not implemented");
}

fn tab_general_init() {
    tab_general_reset();

    if let Ok(mut ui) = CURRENT_UI.write() {
        fltk_set_lazystatic!(ui, input_filename_format, CURRENT_CONFIG, filename_format, tab_general_reset);
        fltk_set_lazystatic!(ui, input_location_format, CURRENT_CONFIG, location_format, tab_general_reset);
        fltk_set_lazystatic!(ui, input_root_directory, CURRENT_CONFIG, location_root, tab_general_reset);
        ui.btn_root_directory.set_callback(move |_| {
            panic!("[btn_root_directory] work in progress");
        });
    }
}
fn tab_general_reset() {
    if let Ok(mut ui) = CURRENT_UI.write() {
        if let Ok(cfg) = CURRENT_CONFIG.read() {
            ui.input_filename_format.set_value(&cfg.filename_format);
            ui.input_location_format.set_value(&cfg.location_format);
            ui.input_root_directory.set_value(&cfg.location_root);
        }
    }
}
fn tab_defaults_init() {
    tab_defaults_reset();
    if let Ok(mut ui) = CURRENT_UI.write() {
        fltk_choice_set_lazystatic!(ui, choice_default_action, CURRENT_CONFIG, default_action, TargetAction, tab_defaults_reset);
        fltk_choice_set_lazystatic!(ui, choice_default_screenshot_type, CURRENT_CONFIG, default_screenshot_type, LScreenshotType, tab_defaults_reset);
        fltk_choice_set_lazystatic!(ui, choice_default_target, CURRENT_CONFIG, default_target, ImageTarget, tab_defaults_reset);
        fltk_choice_set_lazystatic!(ui, choice_default_post_target_action, CURRENT_CONFIG, post_target_action, PostTargetAction, tab_defaults_reset);
        fltk_choice_set_lazystatic!(ui, choice_default_post_upload_action, CURRENT_CONFIG, post_upload_action, PostUploadAction, tab_defaults_reset);
    }
}
fn tab_defaults_reset() {
    if let Ok(mut ui) = CURRENT_UI.write() {
        if let Ok(cfg) = CURRENT_CONFIG.read() {
            TargetAction::populate_choice(&mut ui.choice_default_action);
            LScreenshotType::populate_choice(&mut ui.choice_default_screenshot_type);
            ImageTarget::populate_choice(&mut ui.choice_default_target);

            PostTargetAction::populate_choice(&mut ui.choice_default_post_target_action);
            PostUploadAction::populate_choice(&mut ui.choice_default_post_upload_action);

            cfg.default_action.select_choice(&mut ui.choice_default_action);
            cfg.default_screenshot_type.select_choice(&mut ui.choice_default_screenshot_type);
            cfg.default_target.select_choice(&mut ui.choice_default_target);

            cfg.post_target_action.select_choice(&mut ui.choice_default_post_target_action);
            cfg.post_upload_action.select_choice(&mut ui.choice_default_post_upload_action);
        }
    }
}
fn tab_xbackbone_init() {
    tab_xbackbone_reset();

    if let Ok(mut ui) = CURRENT_UI.write() {
        fltk_set_lazystatic_option!(ui, input_xbackbone_url, xbackbone_config, CURRENT_CONFIG, XBackboneConfig, url, tab_xbackbone_reset);
        fltk_set_lazystatic_option!(ui, input_xbackbone_token, xbackbone_config, CURRENT_CONFIG, XBackboneConfig, token, tab_xbackbone_reset);
        ui.btn_xbackbone_verify.set_callback(move |_| {
            panic!("[btn_xbackbone_verify] work in progress");
        });
    }
}
fn tab_xbackbone_reset() {
    if let Ok(mut ui) = CURRENT_UI.write() {
        if let Ok(x) = CURRENT_CONFIG.read() {
            if let Some(xb) = x.clone().xbackbone_config {
                ui.input_xbackbone_url.set_value(&xb.url);
                ui.input_xbackbone_token.set_value(&xb.token);
            } else {
                ui.input_xbackbone_url.set_value("");
                ui.input_xbackbone_token.set_value("");
            }
        }
    }
}
fn tab_gcs_init() {
    tab_gcs_reset();

    if let Ok(mut ui) = CURRENT_UI.write() {
        fltk_set_lazystatic_option!(ui, cb_gcs_use_default_auth, gcs_config, CURRENT_CONFIG, GCSConfig, use_default, tab_gcs_reset);
        fltk_set_lazystatic_option_withvalue!(ui, input_gcs_auth_cfg_location, gcs_config, CURRENT_CONFIG, GCSConfig, auth_cfg_location, tab_gcs_reset);
        fltk_set_lazystatic_option!(ui, input_gcs_bucket, gcs_config, CURRENT_CONFIG, GCSConfig, bucket, tab_gcs_reset);
        fltk_set_lazystatic_option!(ui, input_gcs_relative_path, gcs_config, CURRENT_CONFIG, GCSConfig, relative_path, tab_gcs_reset);
        ui.input_gcs_public_url_base.set_callback(move |tb| {
            if let Ok(mut x) = CURRENT_CONFIG.write() {
                let mut cfg = x.clone().gcs_config.unwrap_or(GCSConfig::default());
                if tb.value().len() < 1 {
                    cfg.public_url_base = None;
                } else {
                    cfg.public_url_base = Some(tb.value());
                }
                x.gcs_config = Some(cfg);
            }
            tab_gcs_reset();
        });
        fltk_set_lazystatic_option_withvalue!(ui, input_gcs_public_url_base, gcs_config, CURRENT_CONFIG, GCSConfig, public_url_base, tab_gcs_reset);
        ui.cb_gcs_public_url_base.set_callback(move |cb| {
            if let Ok(mut x) = CURRENT_CONFIG.write() {
                let mut cfg = x.clone().gcs_config.unwrap_or(GCSConfig::default());
                cfg.public_url_base = match cb.value() {
                    true => {
                        if let None = cfg.public_url_base {
                            Some(String::new())
                        } else {
                            cfg.public_url_base
                        }
                    },
                    false => None
                };
                x.gcs_config = Some(cfg);
            }
            tab_gcs_reset();
        });
    }
}
fn tab_gcs_reset() {
    if let Ok(mut ui) = CURRENT_UI.write() {
        if let Ok(x) = CURRENT_CONFIG.read() {
            let mut gcs_cfg: GCSConfig = GCSConfig::default();
            if let Some(gcs) = x.clone().gcs_config {
                gcs_cfg = gcs;
            }
            ui.cb_gcs_use_default_auth.set_value(gcs_cfg.clone().use_default);
            ui.input_gcs_auth_cfg_location.set_value(&gcs_cfg.clone().auth_cfg_location.unwrap_or("".to_string()));
            ui.input_gcs_bucket.set_value(&gcs_cfg.bucket);
            ui.input_gcs_relative_path.set_value(&gcs_cfg.relative_path);
            ui.input_gcs_public_url_base.set_value(&gcs_cfg.clone().public_url_base.unwrap_or("".to_string()));
            ui.cb_gcs_public_url_base.set_value(gcs_cfg.clone().public_url_base.is_some());
        }
    }
}