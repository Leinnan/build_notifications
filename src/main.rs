#[macro_use]
extern crate serde_derive;
extern crate confy;
extern crate dirs;
mod config;

use notify_rust::Notification;
use feed_rs::parser;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // open::that("https://rust-lang.org")?;
    // open::that("/home/lyshkin/test.py")?;


    let client = reqwest::Client::new();
    {
        let cfg_file = dirs::config_dir().unwrap().join("build_notifications").join("build_notifications.toml");
        let data_file = dirs::data_local_dir().unwrap().join("build_notifications");
        open::that(cfg_file)?;
        // let cfg_file =  confy::get_configuration_file_path("build_notifications", None)?;
        let cfg: config::NotificationsConfig = confy::load("build_notifications")?;
        let res = client.post(cfg.jenkins_jobs[0].as_str())
        .basic_auth(cfg.username.as_str(),Some(cfg.password.as_str()))
        .send()
        .await?
        .text()
        .await?;

        let feed = parser::parse(res.as_bytes());

        match feed {
            Ok(feed) => {
                println!("{:?}",feed);

                let title = match feed.title {
                    Some(title) => {title.content.clone()}
                    _ => "Some job".to_string()
                };
                let body = format!("Feed date: {}, with {} entries", feed.updated.unwrap_or(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)), feed.entries.len());
                Notification::new()
                .summary(title.as_str())
                .body(body.as_str())
                .icon("network-server-symbolic")
                .show().unwrap();
                
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    Ok(())
}