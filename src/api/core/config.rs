use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_driver: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub db_options: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            db_driver: std::env::var("DB_DRIVER")
                .ok()
                .unwrap_or("mongodb".to_string())
                .to_lowercase(),
            db_host: std::env::var("DB_HOST").expect("env `DB_HOST` must be set"),
            db_port: std::env::var("DB_PORT").ok().unwrap_or("27017".to_string()),
            db_name: std::env::var("DB_NAME").expect("env `DB_NAME` must be set"),
            db_username: std::env::var("DB_USERNAME").ok(),
            db_password: std::env::var("DB_PASSWORD").ok(),
            db_options: std::env::var("DB_OPTIONS").ok(),
        }
    }

    pub fn get_database_uri(&self) -> String {
        let auth_part = match (&self.db_username, &self.db_password) {
            (Some(user), Some(pass)) => format!("{}:{}@", user, pass),
            _ => "".to_string(),
        };
        let options = self.db_options.clone().unwrap_or("".to_string());
        let uri = format!(
            "{}://{}{}:{}/{}{}",
            self.db_driver, auth_part, self.db_host, self.db_port, self.db_name, options
        );
        log::debug!("Database URI: {}", uri);
        uri
    }
}
