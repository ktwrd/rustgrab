pub fn create_uuid() -> String {
    let value = uuid::Uuid::new_v4();
    value.to_string()
}

#[derive(Debug)]
pub enum Error {
    ConfigPathBaseFailure(String),
    // UserConfig failed to unwrap something.
    ConfigUnwrapFailure(crate::config::UserConfigKeyword),
    ConfigIOError(std::io::Error, String),
    
    FromUtf8Error(std::string::FromUtf8Error),
    Json(serde_json::Error),
    IO(std::io::Error),
    Clipboard(arboard::Error),
    
    ErrorCode(usize),
    ErrorCodeE(usize, Box<dyn std::error::Error>),
}

pub fn clipboard_copy_text(text: String) -> Result<(), Error> {
    match arboard::Clipboard::new() {
        Ok(mut v) => {
            match v.set_text(&text) {
                Ok(_) => {
                    println!("clipboard_copy_text: {}", text);
                    let dur = std::time::Duration::from_secs(2);
                    std::thread::sleep(dur);
                    Ok(())
                },
                Err(e) => Err(Error::Clipboard(e))
            }
        },
        Err(e) => Err(Error::Clipboard(e))
    }
}