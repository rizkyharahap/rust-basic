use std::env::set_var;

use config::{Case, Config, Environment, File, FileFormat};
use serde::Deserialize;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_config() {
    let config = Config::builder().build().unwrap();

    assert!(config.get_string("APP_NAME").is_err());
}

#[test]
fn test_config_env() {
    unsafe {
        set_var("DB_HOST", "localhost");
        set_var("DB_PORT", "5432");
        set_var("DB_USER", "rizki");
        set_var("DB_PASSWORD", "password");
    }

    let config = Config::builder()
        .add_source(Environment::default().convert_case(Case::Snake))
        .build()
        .unwrap();

    assert_eq!(config.get_string("db_host").unwrap(), "localhost");
    assert_eq!(config.get_int("db_port").unwrap(), 5432);
    assert_eq!(config.get_string("db_user").unwrap(), "rizki");
    assert_eq!(config.get_string("db_password").unwrap(), "password");
}

#[test]
fn test_config_json() {
    let config = Config::builder()
        .add_source(File::new("application.json", FileFormat::Json))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_int("database.port").unwrap(), 5432);
    assert_eq!(config.get_string("database.name").unwrap(), "my_database");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

#[test]
fn test_config_yaml() {
    let config = Config::builder()
        .add_source(File::new("application.yaml", FileFormat::Yaml))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_int("database.port").unwrap(), 5432);
    assert_eq!(config.get_string("database.name").unwrap(), "my_database");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

#[test]
fn test_config_toml() {
    let config = Config::builder()
        .add_source(File::new("application.toml", FileFormat::Toml))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_int("database.port").unwrap(), 5432);
    assert_eq!(config.get_string("database.name").unwrap(), "my_database");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    name: String,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: i32,
    name: String,
    user: String,
    password: String,
}

#[test]
fn test_config_deserialization() {
    let config = Config::builder()
        .add_source(File::new("application.json", FileFormat::Json))
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app_config.name, "My Application");
    assert_eq!(app_config.database.host, "localhost");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.name, "my_database");
    assert_eq!(app_config.database.user, "user");
    assert_eq!(app_config.database.password, "password");
}
