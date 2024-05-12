mod dialog;
mod image;
mod imgur;
mod mastodon;
mod notification;
mod text;
mod twitter;
mod handler;

use clap::{Arg, Command, crate_version, crate_authors};

#[derive(Copy, Clone)]
pub enum ServiceKind {
    Twitter,
    Mastodon,
    Imgur,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MessageKind {
    Image,
    Text,
}

fn main() {
    // Individual parts the help menu
    let mut locale = text::LocaleValues::new();
    locale.generate();

    let file_arg = Arg::new("file")
            .short('f')
            .long("file")
            .help(locale.File.clone())
            .action(clap::ArgAction::Set)
            .default_missing_value("");

    // Build help menu with clap.rs
    let cmd = Command::new("rustgrab")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Screenshot Utility made with Rust")
        // .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
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
        );

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
        Some(("toot", toot_matches)) => {
            let target_kind = handler::arg_to_kind(toot_matches);
            handler::run(ServiceKind::Mastodon, true, target_file, target_kind);
        },
        Some(("tweet", tweet_matches)) => {
            let target_kind = handler::arg_to_kind(tweet_matches);
            handler::run(ServiceKind::Mastodon, target_file != None, target_file, target_kind);
        },
        Some(("imgur", imgur_matches)) => {
            let target_kind = handler::arg_to_kind(imgur_matches);
            handler::run(ServiceKind::Imgur, target_file != None, target_file, target_kind);
        },
        _ => {
            println!("Nothing provided!");
        },
    };
}