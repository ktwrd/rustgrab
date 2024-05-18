use std::env;
use yaml_rust::{Yaml, YamlLoader};

pub struct LocaleValues {
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
    pub default_action: String,
    pub action_screenshot: String,
    pub action_upload: String
}
fn try_get_yaml(data: &Yaml, failover: &str) -> String {
    data.as_str().unwrap_or(failover).to_string()
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
            default_action: String::new(),
            action_screenshot: String::new(),
            action_upload: String::new()
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
        self.action_screenshot = try_get_yaml(&locator["Action"]["Screenshot"], "<screenshot action>");
        self.action_upload = try_get_yaml(&locator["Action"]["Upload"], "<upload action>");
        self
    }
}

pub fn code() -> String {
    match env::var("LC_CTYPE") {
        Ok(ok) => ok,
        Err(_) => match env::var("LANG") {
            Ok(ok) => ok,
            Err(_) => String::new(),
        },
    }
}
pub fn loader() -> String {
    let lang = code().to_lowercase();
    for x in crate::locale_content::AVAILABLE.into_iter() {
        let contains = match x {
            &"cn" => {
                lang.contains("cn") && lang.contains("zh")
            },
            &"tw" => {
                lang.contains("tw") && lang.contains("zh")
            },
            _ => lang.contains(x)
        };
        if contains {
            let s = x.to_string();
            return crate::locale_content::get_content(s);
        }
    }
    return crate::locale_content::get_content(String::from("en"));
}

pub fn error(code: usize) -> String {
    let r = error_raw(code);
    let (error_txt, msg_txt) = (r.error, r.code);

    match code {
        1..=52 => return format!("{} {}: {}", error_txt, code, msg_txt),
        _ => unreachable!("Internal Logic Error"),
    };
}
pub fn error_msg(code: usize, replacement: String) -> String {
    let d = error(code);
    d.replace("%s", &replacement)
}

pub struct ErrorRawValue {
    error: String,
    code: String
}
impl ErrorRawValue {
    pub fn new(e: String, c: String) -> Self {
        Self {
            error: e,
            code: c
        }
    }
}
pub fn error_raw(code: usize) -> ErrorRawValue {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();
    let error_msg = &locator[code].as_str().unwrap();
    ErrorRawValue::new(error.to_string(), error_msg.to_string())
}

pub fn error_code(code: usize) -> String {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error_msg = &locator[code].as_str().unwrap();
    error_msg.to_string()
}