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
            jenkins_jobs: vec!["none".to_string()],
        }
    }
}
