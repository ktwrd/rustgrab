
use clap::{
    crate_authors, crate_version, Arg, Command};

use rustgrab::{
    locale::LocaleValues,
    handler
};


#[tokio::main]
#[allow(unused_assignments, unused_variables)]
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
        .subcommand(
            Command::new("upload")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.action_upload.clone())
                .arg(
                    Arg::new("file")
                    .help(locale.file.clone())
                    .action(clap::ArgAction::Set)
                )
        )
        .subcommand(
            Command::new("config")
                .version(crate_version!())
                .author(crate_authors!())
                .about(locale.action_config.clone())
                .subcommand(Command::new("init")
                    .about(locale.action_config_init.clone()))
                .subcommand(Command::new("open")
                    .about(locale.action_config_open.clone()))
                .subcommand(Command::new("location")
                    .about(locale.action_config_location.clone()))
        )
        .subcommand(
            Command::new("devtest")
                .subcommand(Command::new("progress_dialog"))
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

    match cmd.clone().get_matches().subcommand() {
        Some(("default", _)) => {
            handler::run_default_cfg().await;
        },
        Some(("screenshot", screenshot_matches)) => {
            rustgrab::action::screenshot::run(screenshot_matches).await;
        },
        Some(("upload", upload_matches)) => {
            rustgrab::action::upload::run(upload_matches).await;
        },
        Some(("config", config_matches)) => {
            match config_matches.subcommand_name() {
                Some("init") => rustgrab::action::config::init().await,
                Some("open") => rustgrab::action::config::open().await,
                Some("location") => rustgrab::action::config::display_location().await,
                _ => println!("Unknown subcommand")
            }
        },
        Some(("devtest", dt_matches)) => {
            match dt_matches.subcommand_name() {
                Some("progress_dialog") => rustgrab::handler::progress_dialog::test().await,
                _ => panic!("no sub-command or invalid subcommand")
            }
        }
        _ => {
            println!("Nothing provided or sub-command is not supported!");
        },
    };
}