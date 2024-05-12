use crate::{text, config::ImageTarget, notification::NotificationKind};
use reqwest::blocking::multipart;

pub fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), crate::helper::Error> {
    let xb_cfg = match config.xbackbone_config {
        Some(ref v) => v,
        None => {
            eprintln!("{}", text::message(36).replace("%s", "XBackbone"));
            crate::notification::error_msg(36, "XBackbone".to_string());
            crate::text::exit();
        }
    };
    
    let target_location = config.generate_location()?.clone();
    let filename = &target_location.split('/').into_iter().last().unwrap();
    let filename_str = filename.to_string();
    if crate::image::image_to_file(kind, target_location.clone()) == false {
        eprintln!("{}", text::message(30));
        crate::text::exit();
    }
    
    let content = match std::fs::read(target_location.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} {}", text::message(19), target_location.clone());
            eprintln!("{:#?}", e);
            crate::notification::error_body(19, format!("{}", e));
            crate::text::exit();
        }
    };
    
    let content_part = match multipart::Part::bytes(content).file_name(filename_str).mime_str("image/png") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to set mime type to image/png");
            eprintln!("{:#?}", e);
            crate::notification::error_body(44, format!("{}", e));
            crate::text::exit();
        }
    };
    let form = multipart::Form::new()
        .text("token", format!("{}", xb_cfg.token))
        .part("upload", content_part);
    
    let client = reqwest::blocking::Client::new();
    let response = match client.post(format!("{}", xb_cfg.url)).multipart(form).send() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("handler.xbackbone.run->response {:#?}", e);
            crate::notification::error(37);
            crate::text::exit();
        }
    };
    let status = response.status();
    match status {
        reqwest::StatusCode::CREATED => {
            let txt = response.text();
            let response_body = match txt {
                Ok(ref v) => v,
                Err(e) => {
                    eprintln!("handler.xbackbone.run->response_body {:#?}", e);
                    crate::notification::error(38);
                    crate::text::exit();
                }
            };
            println!("response_body: {}", response_body);
            let response_data: XBackboneResponse = match serde_json::from_str(response_body.as_str()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("handler.xbackbone.run->response_data {:#?}", e);
                    crate::notification::error(40);
                    crate::text::exit();
                }
            };
            if response_data.message != "OK".to_string() {
                eprintln!("handler.xbackbone.run->response_data unhandled message {}", response_data.message);
                eprintln!("{:#?}", response_data);
                crate::notification::error(41);
                crate::text::exit();
            }
            
            match crate::helper::clipboard_copy_text(response_data.url) {
                Ok(_) => {
                    crate::notification::display(ImageTarget::XBackbone, NotificationKind::ClipboardCopy);
                    Ok(())
                },
                Err(e) => {
                    println!("failed to copy to clipboard: {:#?}", e);
                    eprintln!("{}", crate::text::message(42));
                    crate::notification::error(42);
                    Err(e)
                }
            }
        },
        _ => {
            eprintln!("handler.xbackbone.run->status {:#?}", status);
            crate::notification::error(39);
            crate::text::exit();
        }
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct XBackboneConfig {
    pub token: String,
    pub url: String
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct XBackboneResponse {
    pub message: String,
    pub version: String,
    pub url: String
}