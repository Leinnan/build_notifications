#[macro_use]
extern crate serde_derive;
extern crate confy;
extern crate dirs;
mod config;
mod rss_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rss_data = crate::rss_data::RssData::default();

    {use clap::{App, Arg};
    let matches = App::new("build_notifications")
        .version("0.1")
        .author("Piotr S. <mev_lyshkin@protonmail.com>")
        .about("Simple job checker")
        .arg(
            Arg::with_name("config")
                .short("c")
                .help("open config file and exit"),
        )
        .get_matches();

    if matches.is_present("config") {
        let cfg_file = dirs::config_dir()
            .unwrap()
            .join("build_notifications")
            .join("build_notifications.toml");
        open::that(cfg_file)?;
        return Ok(());
    }}

    loop {
        use tokio::time::{sleep, Duration};

        rss_data.run_checks().await?;

        sleep(Duration::from_millis(180000)).await;
    }
}
