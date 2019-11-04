//! Utility module acting as a way to access a limited set of environment
//! variables at runtime.
use std::env;

lazy_static::lazy_static! {
    pub static ref CLIENT_HOSTNAME: String = env::var("CLIENT_HOSTNAME").unwrap();
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
    pub static ref GITHUB_APP_ID: String = env::var("GITHUB_APP_ID").unwrap();
    pub static ref GITHUB_ID: String = env::var("GITHUB_ID").unwrap();
    pub static ref GITHUB_SECRET: String = env::var("GITHUB_SECRET").unwrap();
    pub static ref REDIRECT_URI: String = format!("http://{}/authorise", &*SERVER_HOSTNAME);
    pub static ref REDIS_URL: String = env::var("REDIS_URL").unwrap();
    pub static ref SECRET_KEY: Vec<u8> = env::var("SECRET_KEY").unwrap().into_bytes();
    pub static ref SERVER_HOSTNAME: String = env::var("SERVER_HOSTNAME").unwrap();
}
