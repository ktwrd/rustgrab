use crate::config::UserConfig;

pub async fn open() {
    let location = UserConfig::get_config_location();
    let location_path = std::path::Path::new(location.as_str());
    edit::edit_file(location_path).expect("Failed to open config file!");
}
pub async fn init() {
    todo!("init wizard for config file");
}
pub async fn display_location() -> ! {
    let location = UserConfig::get_config_location();
    println!("{}", location);
    std::process::exit(0)
}