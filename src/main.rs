#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use config::{Config, ConfigError, Environment};
use reqwest::Client;
use rocket::response::status::BadRequest;
use rocket::State;

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    app_token: String,
    gotify_url: String,
}

impl AppConfig {
    fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix("gap"))
            .build()?;
        s.try_deserialize()
    }
}

#[post("/", data = "<data>")]
async fn proxy(
    data: String,
    config: &State<AppConfig>,
    client: &State<Client>,
) -> Result<(), BadRequest<String>> {
    let response = client
        .post(format!(
            "{}/message?token={}",
            config.gotify_url, config.app_token
        ))
        .body(data)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;
    response
        .error_for_status()
        .map(|_| ())
        .map_err(|e| BadRequest(Some(e.to_string())))
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![proxy, health])
        .manage(reqwest::Client::new())
        .manage(AppConfig::new().expect("Could not read config from environment"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        std::env::set_var("GAP_APP_TOKEN", "token");
        std::env::set_var("GAP_GOTIFY_URL", "url");

        let c = AppConfig::new().unwrap();

        assert_eq!(c.app_token, "token".to_string());
        assert_eq!(c.gotify_url, "url".to_string());
    }
}
