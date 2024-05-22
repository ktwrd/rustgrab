use clap::ArgMatches;
use screenshot_rs::ScreenshotKind;

pub async fn run(matches: &ArgMatches) {
    let kind = match matches.subcommand_name() {
        Some("area") => Some(ScreenshotKind::Area),
        Some("window") => Some(ScreenshotKind::Window),
        Some("full") => Some(ScreenshotKind::Full),
        _ => None
    };

    match kind {
        Some(v) => {
            crate::handler::run_screenshot_cfg(None, v).await;
        },
        None => {
            println!("No action provided");
            crate::msgbox::error(52);
        }
    }
}