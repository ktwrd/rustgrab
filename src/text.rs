use std::{env, process};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
pub enum Errors {
    GtkInitError,
    ScreenshotUnsuccessful,
    StrfTimeUnavailable,
    PicturesFolderUnavailable,
    ScreenshotSaveError,
    ImageViewerError,
    FileError,
    ImgurUploadFailure,
    ClipboardUnavailable,
    MastodonImageUnsuccessful,
    MastodonTootUnsuccessful,
    MastodonLoginError,
    TwitterImageUnsuccessful,
    TwitterTweetUnsucessful,
    TwitterLoginError,
    NotificationUnavailable,
}

#[allow(dead_code)]
pub enum Text {
    UnsupportedWayland,
}

pub struct LocaleValues
{
    pub Help: String,
    pub Version: String,
    pub Area: String,
    pub Window: String,
    pub Full: String,
    pub File: String,
    pub Mastodon: String,
    pub Twitter: String,
    pub Imgur: String,
    pub TwitterAuth: String,
    pub MastodonAuth: String
}
impl LocaleValues {
    pub fn new() -> Self
    {
        Self {
            Help: String::new(),
            Version: String::new(),
            Area: String::new(),
            Window: String::new(),
            Full: String::new(),
            File: String::new(),
            Mastodon: String::new(),
            Twitter: String::new(),
            Imgur: String::new(),
            TwitterAuth: String::new(),
            MastodonAuth: String::new()
        }
    }
    pub fn generate(&mut self) -> &mut Self
    {
        let locators = YamlLoader::load_from_str(&loader()).unwrap();
        let locator = &locators[0]["Help"].clone();
        self.Help = locator["Help"].as_str().unwrap_or("<help>").clone().to_string();
        self.Version = locator["Version"].as_str().unwrap_or("<version>").clone().to_string();
        self.Area = locator["Area"].as_str().unwrap_or("<area>").clone().to_string();
        self.Window = locator["Window"].as_str().unwrap_or("<window>").clone().to_string();
        self.Full = locator["Full"].as_str().unwrap_or("<full>").clone().to_string();
        self.File = locator["File"].as_str().unwrap_or("<file>").clone().to_string();
        self.Mastodon = locator["Toot"].as_str().unwrap_or("<mastodon>").clone().to_string();
        self.Twitter = locator["Tweet"].as_str().unwrap_or("<twitter>").clone().to_string();
        self.Imgur = locator["Imgur"].as_str().unwrap_or("<imgur>").clone().to_string();
        self.TwitterAuth = locator["Twitter"]["Auth"].as_str().unwrap_or("<twitter_auth>").clone().to_string();
        self.MastodonAuth = locator["Mastodon"]["Auth"].as_str().unwrap_or("<mastodon_auth>").clone().to_string();
        self
    }
}

// Retrieves locale settings of the user using LC_CTYPE or LANG
fn locale() -> String {
    match env::var("LC_CTYPE") {
        Ok(ok) => ok,
        Err(_) => match env::var("LANG") {
            Ok(ok) => ok,
            Err(_) => String::new(),
        },
    }
}

// Retrieves the correct localization file and returns a String
pub fn loader() -> String {
    let lang = locale();

    if lang.contains("fr") {
        return include_str!("../lang/fr.yml").to_string();
    } else if lang.contains("es") {
        return include_str!("../lang/es.yml").to_string();
    } else if lang.contains("eo") {
        return include_str!("../lang/eo.yml").to_string();
    } else if lang.contains("CN") && lang.contains("zh") {
        return include_str!("../lang/cn.yml").to_string();
    } else if lang.contains("TW") && lang.contains("zh") {
        return include_str!("../lang/tw.yml").to_string();
    } else if lang.contains("ja") {
        return include_str!("../lang/ja.yml").to_string();
    } else if lang.contains("ko") {
        return include_str!("../lang/ko.yml").to_string();
    } else if lang.contains("de") {
        return include_str!("../lang/de.yml").to_string();
    } else if lang.contains("pl") {
        return include_str!("../lang/pl.yml").to_string();
    } else if lang.contains("pt") {
        return include_str!("../lang/pt.yml").to_string();
    } else if lang.contains("sv") {
        return include_str!("../lang/sv.yml").to_string();
    } else if lang.contains("tr") {
        return include_str!("../lang/tr.yml").to_string();
    } else {
        return include_str!("../lang/en.yml").to_string();
    }
}

// Ends the current process with an error code (useful in BASH scripting)
pub fn exit() -> ! {
    process::exit(1);
}

// Gets error message from appropriate localization file provided by language::loader()
// and returns it as a String
pub fn message(code: usize) -> String {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();
    match code {
        1..=31 => return format!("{} {}: {}", error, code, &locator[code].as_str().unwrap()),
        _ => unreachable!("Internal Logic Error"),
    };
}
