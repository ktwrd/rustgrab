use crate::config::{ImageTarget, UserConfig};
use crate::{notification, locale, LError};
use crate::handler::{TargetResultData, TargetResultUploadData};

pub async fn run(config: crate::config::UserConfig, kind: screenshot_rs::ScreenshotKind)
    -> Result<TargetResultData, LError>{

    let location = config.generate_location()?;
    
    if crate::image_to_file(kind, location.clone()) == false {
        eprintln!("{}", locale::error(30));
        return Err(LError::ErrorCode(30));
    }

    upload(config.clone(), location).await
}

pub async fn upload(config: UserConfig, location: String)
    -> Result<TargetResultData, LError>
{
    let im_client_id = match &config.imgur_config {
        Some(v) => v.client_id.clone(),
        None => {
            eprintln!("[handler::imgur::run] no client_id in config, using own.");
            String::from(DEFAULT_CLIENT_ID)
        }
    };

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

    let loc_str = location.clone();
    // Send notification
    notification::image_sent(ImageTarget::Imgur, &imgur_result.data.link, loc_str.as_str());

    // Copy url to clipboard
    let link = imgur_result.data.link.clone();
    Ok(TargetResultData::Upload(TargetResultUploadData
    {
        fs_location: loc_str,
        url: link
    }))
}

pub const DEFAULT_CLIENT_ID: &str = include_str!("imgur_client_id.txt");

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ImgurConfig {
    pub client_id: String
}