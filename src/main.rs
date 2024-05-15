
use clap::{
    Command,
    crate_version,
    crate_authors};

use rustgrab::{
    locale::LocaleValues,
    handler
};


#[tokio::main]
async fn main() {
    // Individual parts the help menu
    let mut locale = LocaleValues::new();
    locale.generate();

    /*let file_arg = Arg::new("file")
            .short('f')
            .long("file")
            .help(locale.File.clone())
            .action(clap::ArgAction::Set)
            .default_missing_value("");*/

    // Build help menu with clap.rs
    let cmd = Command::new("rustgrab")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Screenshot Utility made with Rust")
        // .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
            Command::new("default")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.default_action.clone())
                .subcommand(Command::new("area").about(locale.area.clone()))
                .subcommand(Command::new("window").about(locale.window.clone()))
                .subcommand(Command::new("full").about(locale.full.clone()))
        )
        .subcommand(
            Command::new("screenshot")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.action_screenshot.clone())
                .subcommand(Command::new("area").about(locale.area.clone()))
                .subcommand(Command::new("window").about(locale.window.clone()))
                .subcommand(Command::new("full").about(locale.full.clone()))
        )
        /*.subcommand(
            Command::new("toot")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.Mastodon.clone())
                .arg(file_arg.clone())
                .subcommand(Command::new("auth").about(locale.MastodonAuth.clone()))
                .subcommand(Command::new("area").about(locale.Area.clone()))
                .subcommand(Command::new("window").about(locale.Window.clone()))
                .subcommand(Command::new("full").about(locale.Full.clone())),
        )
        .subcommand(
            Command::new("tweet")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.Twitter.clone())
                .arg(file_arg.clone())
                .subcommand(Command::new("auth").about(locale.TwitterAuth.clone()))
                .subcommand(Command::new("area").about(locale.Area.clone()))
                .subcommand(Command::new("window").about(locale.Window.clone()))
                .subcommand(Command::new("full").about(locale.Full.clone())),
        )
        .subcommand(
            Command::new("imgur")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.Imgur.clone())
                .arg(file_arg.clone())
                .subcommand(Command::new("area").about(locale.Area.clone()))
                .subcommand(Command::new("window").about(locale.Window.clone()))
                .subcommand(Command::new("full").about(locale.Full.clone())),
        )*/;

    let mut target_file: Option<String> = None;
    for a in cmd.clone().get_arguments().into_iter() {
        let a_id_str = a.get_id().to_string();
        if a_id_str == "file".to_string() {
            if a.get_index() != None
            {
                println!("Using file at {}", a.to_string());
                target_file = Some(a.to_string());
            }
        }
    }
    match cmd.clone().get_matches().subcommand() {
        Some(("default", _)) => {
            handler::run_default_cfg().await;
        },
        Some(("screenshot", screenshot_matches)) => {
            match handler::arg_to_kind(screenshot_matches) {
                Some(v) => {
                    handler::run_screenshot_cfg(None, v).await;
                },
                None => {
                    println!("No action provided");
                }
            }
        }
        _ => {
            println!("Nothing provided or sub-command is not supported!");
        },
    };
}