use crate::{config::{ImageTarget, PostUploadAction, UserConfig}, notification::NotificationKind, LError};

use super::TargetResultUploadData;

pub struct PostUploadActionHandler
{
    pub config: UserConfig,
    pub target: ImageTarget
}

impl PostUploadActionHandler
{
    pub fn run(&self, data: TargetResultUploadData) -> Result<(), LError> {
        match self.config.post_upload_action {
            PostUploadAction::CopyLink => self.copy_url(data.url),
            _ => panic!("Unhandled action {:#?}", self.config.post_upload_action)
        }
    }
    pub fn copy_url(&self, url: String) -> Result<(), LError> {
        match crate::clipboard::copy_text(url) {
            Ok(_) => {
                crate::notification::display(self.target, NotificationKind::ClipboardCopy);
                Ok(())
            },
            Err(e) => {
                crate::notification::error(42);
                Err(e)
            }
        }
    }
}