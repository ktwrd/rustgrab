use clap::ArgMatches;

pub async fn run(matches: &ArgMatches) {
    match matches.get_one::<String>("file") {
        Some(v) => file(v.clone()).await,
        None => {
            println!("No file provided");
            crate::msgbox::error(51);
        }
    }
}
pub async fn file(location: String) {
    crate::handler::run_default_upload_cfg(location).await;
}