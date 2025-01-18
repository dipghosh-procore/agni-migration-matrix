// use std::str::FromStr;
use url::Url;


pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub db_name: String,
    pub app_name: String,
}

impl Config {
    pub fn parse_db_url(db_url: &str, app_name: &str) -> Result<Self, String> {

        let url = Url::parse(db_url).map_err(|e| format!("Invalid database URL: {}", e))?;
        
        let db_name = url
            .path_segments()
            .and_then(|segments| segments.last())
            .ok_or("Database name is missing in the database URL.".to_string())?
            .to_string();

        Ok(Self {
            host: url.host_str().unwrap_or("").to_string(),
            port: url.port().unwrap_or(5432),
            user: url.username().to_string(),
            password: Some(url.password().unwrap_or("").to_string()),
            db_name: db_name,
            app_name: app_name.to_string(),
        })

    }
}