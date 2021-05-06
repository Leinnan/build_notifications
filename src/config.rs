#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationsConfig {
    pub username: String,
    pub password: String,
    pub jenkins_jobs: Vec<String>,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            username: "none".to_string(),
            password: "none".to_string(),
            jenkins_jobs: vec!["https://www.reddit.com/r/ps1graphics/.rss".to_string()],
        }
    }
}

impl NotificationsConfig {
    pub fn config_path() -> std::path::PathBuf {
        dirs::config_dir()
            .unwrap()
            .join("build_notifications")
            .join("build_notifications.toml")
    }

    pub fn read_from_file() -> Self {
        confy::load_path(Self::config_path()).unwrap()
    }

}
