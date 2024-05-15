use crate::config::ImageTarget;
use crate::{notification, locale, LError};

pub async fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<(), LError>{
    let im_client_id = match &config.imgur_config {
        Some(v) => v.client_id.clone(),
        None => {
            eprintln!("[handler::imgur::run] no client_id in config, using own.");
            String::from(DEFAULT_CLIENT_ID)
        }
    };

    let location = config.generate_location()?;
    
    if crate::image_to_file(kind, location.clone()) == false {
        eprintln!("{}", locale::error(30));
        return Ok(());
    }

    let imgur_client = imgurs::ImgurClient::new(&im_client_id.clone());
    let imgur_result = match imgur_client.upload_image(&location).await {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::Imgur(e));
        }
    };

    if imgur_result.success == false {
        return Err(LError::ImgurFailure(imgur_result));
    }

    // Send notification
    notification::image_sent(ImageTarget::Imgur, &imgur_result.data.link, location.as_str());

    // Copy url to clipboard
    crate::clipboard::copy_text(imgur_result.data.link)
}

pub const DEFAULT_CLIENT_ID: &str = include_str!("imgur_client_id.txt");

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ImgurConfig {
    pub client_id: String
}