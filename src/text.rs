use std::{env, process};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
pub enum ErrorCodenames {
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

    UnwrapBaseConfigLocation,
    UnwrapFinalConfigLocation,
    FallbackDirectoryFail
}

#[allow(dead_code)]
pub enum Text {
    UnsupportedWayland,
}

pub struct LocaleValues
{
    pub help: String,
    pub version: String,
    pub area: String,
    pub window: String,
    pub full: String,
    pub file: String,
    pub mastodon: String,
    pub twitter: String,
    pub imgur: String,
    pub twitter_auth: String,
    pub mastodon_auth: String,
    pub default_action: String
}
impl LocaleValues {
    pub fn new() -> Self
    {
        Self {
            help: String::new(),
            version: String::new(),
            area: String::new(),
            window: String::new(),
            full: String::new(),
            file: String::new(),
            mastodon: String::new(),
            twitter: String::new(),
            imgur: String::new(),
            twitter_auth: String::new(),
            mastodon_auth: String::new(),
            default_action: String::new()
        }
    }
    pub fn generate(&mut self) -> &mut Self
    {
        let locators = YamlLoader::load_from_str(&loader()).unwrap();
        let locator = &locators[0]["Help"].clone();
        self.help = locator["Help"].as_str().unwrap_or("<help>").to_string();
        self.version = locator["Version"].as_str().unwrap_or("<version>").to_string();
        self.area = locator["Area"].as_str().unwrap_or("<area>").to_string();
        self.window = locator["Window"].as_str().unwrap_or("<window>").to_string();
        self.full = locator["Full"].as_str().unwrap_or("<full>").to_string();
        self.file = locator["File"].as_str().unwrap_or("<file>").to_string();
        self.mastodon = locator["Toot"].as_str().unwrap_or("<mastodon>").to_string();
        self.twitter = locator["Tweet"].as_str().unwrap_or("<twitter>").to_string();
        self.imgur = locator["Imgur"].as_str().unwrap_or("<imgur>").to_string();
        self.twitter_auth = locator["Twitter"]["Auth"].as_str().unwrap_or("<twitter_auth>").to_string();
        self.mastodon_auth = locator["Mastodon"]["Auth"].as_str().unwrap_or("<mastodon_auth>").to_string();
        self.default_action = locator["DefaultAction"].as_str().unwrap_or("<default action from config>").to_string();
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
    let (error_txt, msg_txt) = message_raw(code);

    match code {
        1..=47 => return format!("{} {}: {}", error_txt, code, msg_txt),
        _ => unreachable!("Internal Logic Error"),
    };
}
// Gets the specified code from the localization files
// Returns (Error.Error, Error.[code])
pub fn message_raw(code: usize) -> (String, String) {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();
    let error_msg = &locator[code].as_str().unwrap();
    (error.to_string(), error_msg.to_string())
}
pub fn message_code(code: usize) -> String {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error_msg = &locator[code].as_str().unwrap();
    error_msg.to_string()
}
