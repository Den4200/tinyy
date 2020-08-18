use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;


/// Creates rocket config from environment variables.
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found.");

    let address = env::var("address")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable not found.");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not found.");

    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .address(address)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
