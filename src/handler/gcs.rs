use crate::{config::UserConfig, LError};
use crate::handler::{TargetResultData, TargetResultUploadData};
use futures::StreamExt;
use indicatif::ProgressBar;
use tokio_util::io::ReaderStream;

pub async fn screenshot(config: UserConfig, kind: ScreenshotKind)
    -> Result<TargetResultData, LError>
{
    let target_location = config.generate_location()?.clone();
    let img_res = crate::image_to_file(kind, target_location.clone());
    println!("[handler::gcs::screenshot] image_res: {}", img_res);
    if img_res == false {
        return Err(LError::ErrorCode(30));
    }
    upload(config, target_location).await
}

pub async fn upload(config: UserConfig, location: String)
    -> Result<TargetResultData, LError>
{
    let gcs_cfg = match config.gcs_config {
        Some(ref v) => v,
        None => {
            return Err(LError::ErrorCode(36))
        }
    };
    
    let filename = &location.split('/').into_iter().last().unwrap();
    
    let mut relative_location = gcs_cfg.relative_path.clone();
    if relative_location.ends_with("/") == false {
        relative_location.push_str("/");
    }
    relative_location.push_str(filename);
    if relative_location.starts_with("/") {
        relative_location.pop();
    }
    let current_date = chrono::Local::now();
    relative_location = crate::config::UserConfig::format_location(relative_location.clone(), &current_date);

    crate::notification::debug(String::from("Uploading"));

    let client = create_gcs_client(config.clone()).await?;

    let content_type = crate::helper::get_content_type(relative_location.clone());
    let upload_type = UploadType::Simple(Media
    {
        name: relative_location.clone().into(),
        content_type: content_type.into(),
        content_length: None
    });

    let file = match tokio::fs::File::open(location.clone()).await {
        Ok(v) => v,
        Err(e) => {
            return Err(LError::ErrorCodeMsg(19, format!("{:#?}", e)));
        }
    };
    let total_size = match file.metadata().await {
        Ok(v) => v.len(),
        Err(e) => {
            return Err(LError::ErrorCodeMsg(19, format!("{:#?}", e)));
        }
    };
    let mut reader_stream = ReaderStream::new(file);
    let mut uploaded = 0 as u64;

    let pb = ProgressBar::new(total_size);
    pb.set_style(indicatif::ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let async_stream = async_stream::stream! {
        while let Some(chunk) = reader_stream.next().await {
            if let Ok(chunk) = &chunk {
                let new = std::cmp::min(uploaded + (chunk.len() as u64), total_size);
                uploaded = new;
                pb.set_position(new);
                if uploaded >= total_size {
                    pb.finish_with_message("done!");
                }
            }
            yield chunk;
        }
    };

    let data = client.upload_streamed_object(&UploadObjectRequest {
        bucket: gcs_cfg.bucket.clone(),
        ..Default::default()
    }, async_stream, &upload_type).await;

    match data {
        Ok(_) => {
            let mut base_url = gcs_cfg.public_url_base.clone().unwrap_or(String::from(DEFAULT_BASE_URL));
            base_url = base_url.replace("$bucket", &gcs_cfg.bucket);
            if base_url.ends_with('/') {
                base_url.pop();
            }
            
            Ok(TargetResultData::Upload(TargetResultUploadData{
                fs_location: location,
                url: format!("{}/{}", base_url, relative_location)
            }))
        },
        Err(e) => {
            eprintln!("[handler::gcs::get_gcs_config] failed to upload file");
            Err(LError::GoogleCloudStorageHttp(e))
        }
    }
}
use google_cloud_storage::client::Client as GCSClient;
use google_cloud_storage::client::ClientConfig as GCSClientConfig;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use screenshot_rs::ScreenshotKind;
async fn create_gcs_client(config: UserConfig) -> Result<GCSClient, LError> {
    let c = get_gcs_config(config.clone()).await?;
    let client = GCSClient::new(c);
    Ok(client)
}
async fn get_gcs_config(config: UserConfig) -> Result<GCSClientConfig, LError> {
    let gcs_cfg = match config.gcs_config {
        Some(ref v) => v,
        None => {
            eprintln!("[handler::gcs::get_gcs_config] gcs_config is none");
            return Err(LError::ErrorCode(36))
        }
    };
    
    match gcs_cfg.clone().use_default {
        true => match GCSClientConfig::default().with_auth().await {
            Ok(v) => Ok(v),
            Err(e) => {
                eprintln!("[handler::gcs::get_gcs_config] failed to get default via with_auth");
                Err(LError::GoogleCloudAuth(e))
            }
        },
        false => {
            match gcs_cfg.clone().auth_cfg_location {
                Some(v) => {
                    match google_cloud_auth::credentials::CredentialsFile::new_from_file(v.clone()).await {
                        Ok(auth) => {
                            match GCSClientConfig::default().with_credentials(auth).await {
                                Ok(v) => Ok(v),
                                Err(e) => {
                                    eprintln!("[handler::gcs::get_gcs_config] failed to create config with credentials at the location provided");
                                    Err(LError::GoogleCloudAuth(e))
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("[handler::gcs::get_gcs_config] failed to create credentials from {}", v);
                            Err(LError::GoogleCloudAuth(e))
                        }
                    }
                },
                None => {
                    eprintln!("[handler::gcs::get_gcs_config] auth_cfg_location is none but use_default is false!");
                    Err(LError::ErrorCode(50))
                }
            }
        }
    }
}

pub const DEFAULT_BASE_URL: &str = "https://storage.googleapis.com/$bucket";
pub enum GCSAuthFrom {
    Default,
    SALocation,

}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GCSConfig {
    pub use_default: bool,
    pub auth_cfg_location: Option<String>,

    pub bucket: String,
    pub relative_path: String,
    pub public_url_base: Option<String>
}
impl Default for GCSConfig {
    fn default() -> Self {
        Self {
            use_default: true,
            auth_cfg_location: None,
            bucket: String::new(),
            relative_path: String::from("upload/$rand"),
            public_url_base: None
        }
    }
}
