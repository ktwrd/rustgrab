use crate::{LError, config::ImageTarget, notification::NotificationKind};
use reqwest::blocking::multipart;

pub fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), LError> {
    let xb_cfg = match config.xbackbone_config {
        Some(ref v) => v,
        None => {
            return Err(LError::ErrorCode(36));
        }
    };
    
    let target_location = config.generate_location()?.clone();
    let filename = &target_location.split('/').into_iter().last().unwrap();
    let filename_str = filename.to_string();
    if crate::image_to_file(kind, target_location.clone()) == false {
        return Err(LError::ErrorCode(30));
    }
    
    let content = match std::fs::read(target_location.clone()) {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::ErrorCodeMsg(19, format!("{}", e)));
        }
    };
    
    let content_part = match multipart::Part::bytes(content).file_name(filename_str).mime_str("image/png") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to set mime type to image/png");
            eprintln!("{:#?}", e);
            return Err(LError::ErrorCodeMsg(44, format!("{}", e)));
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
            return Err(LError::ErrorCode(37));
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
                    return Err(LError::ErrorCode(38));
                }
            };
            let response_data: XBackboneResponse = match serde_json::from_str(response_body.as_str()) {
                Ok(v) => v,
                Err(e) => {
                    println!("handler.xbackbone.run->response_body {:#?}", response_body);
                    eprintln!("handler.xbackbone.run->response_data {:#?}", e);
                    return Err(LError::ErrorCode(40));
                }
            };
            if response_data.message != "OK".to_string() {
                println!("handler.xbackbone.run->response_data {:#?}", response_data);
                eprintln!("handler.xbackbone.run->response_data unhandled message code {}", response_data.message);
                return Err(LError::ErrorCodeMsg(41, format!("{}", response_data.message)));
            }
            
            match crate::clipboard::copy_text(response_data.url) {
                Ok(_) => {
                    crate::notification::display(ImageTarget::XBackbone, NotificationKind::ClipboardCopy);
                    Ok(())
                },
                Err(e) => {
                    println!("failed to copy to clipboard: {:#?}", e);
                    eprintln!("{}", crate::locale::error(42));
                    crate::notification::error(42);
                    Err(e)
                }
            }
        },
        _ => {
            eprintln!("handler.xbackbone.run->status {:#?}", status);
            return Err(LError::ErrorCodeMsg(39, format!("{:#?}", status)));
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