use crate::config::NotificationsConfig;
use chrono::{DateTime, NaiveDateTime, Utc};
use feed_rs::parser;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{collections::HashMap, path::PathBuf};

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RssData {
    pub update_times: HashMap<String, i64>,
    #[serde(skip_deserializing, skip_serializing)]
    cfg: NotificationsConfig,
}

impl Default for RssData {
    fn default() -> Self {
        let data_file = RssData::path();
        let cfg: NotificationsConfig = confy::load("build_notifications").unwrap();
        let update_times = if data_file.exists() {
            let serialized = std::fs::read_to_string(data_file).unwrap_or_default();
            let deserialized: Self = serde_json::from_str(&serialized).unwrap();
            deserialized.update_times
        } else {
            HashMap::new()
        };

        for job in &cfg.jenkins_jobs {
            println!("{}", job);
        }

        Self { update_times, cfg }
    }
}

impl RssData {
    pub fn path() -> PathBuf {
        dirs::data_local_dir().unwrap().join("build_notifications")
    }

    pub async fn run_checks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let name = self.cfg.username.as_str();
        let password = self.cfg.password.as_str();

        for job in self.cfg.jenkins_jobs.clone() {
            let res = client
                .post(job.as_str())
                .basic_auth(name, Some(password))
                .send()
                .await?
                .text()
                .await?;
            let feed = parser::parse(res.as_bytes());

            match feed {
                Ok(feed) => {
                    let title = match feed.title {
                        Some(title) => title.content.clone(),
                        _ => "Some job".to_string(),
                    };
                    let date = feed
                        .updated
                        .unwrap_or(DateTime::<Utc>::from_utc(
                            NaiveDateTime::from_timestamp(0, 0),
                            Utc,
                        ))
                        .timestamp();

                    match self.update_times.get(&job) {
                        Some(val) if &date <= val => {}
                        _ => {
                            let body = format!(
                                "Update: {}",
                                feed.updated.unwrap_or(DateTime::<Utc>::from_utc(
                                    NaiveDateTime::from_timestamp(0, 0),
                                    Utc
                                ))
                            );
                            Notification::new()
                                .summary(title.as_str())
                                .body(body.as_str())
                                .sound_name(SOUND)
                                .icon("network-server-symbolic")
                                .show()
                                .unwrap();
                        }
                    }
                    self.update_times.insert(job, date);
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }
        self.save();
        Ok(())
    }

    fn save(&self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = std::fs::File::create(Self::path()).unwrap();
        let _ = file.write_all(serialized.as_bytes());
    }
}
