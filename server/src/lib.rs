#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;

pub mod env;
pub mod error;
pub mod events;
pub mod github;
pub mod middleware;
pub mod models;
pub mod users;

#[allow(unused_imports)]
mod schema;

use actix_web::{web, HttpResponse, Responder};
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::Commands;
use reqwest::StatusCode;
use serde::Deserialize;
use snafu::ResultExt;

pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

/// A wrapper struct containing the various client side connections the server
/// makes.
#[derive(Clone)]
pub struct Clients {
    /// Postgres for storing event data.
    pub pg: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    /// Redis for caching requests.
    pub redis: r2d2::Pool<RedisConnectionManager>,
    /// HTTP client to make requests to GitHub's API.
    // FIXME: This is reqwest currently as there's no way to execute async
    // functions in `actix-web` without the whole handler becoming async. Fix me
    // there is are async diesel and redis clients or a way to call something
    // like `actix_rt::SystemRunner::block_on` within the `actix-web` executor.
    pub http: reqwest::Client,
}

#[derive(Deserialize)]
pub struct ProfileRequest {
    #[serde(deserialize_with = "comma_sep_string")]
    names: Vec<String>,
}

fn comma_sep_string<'de, D>(de: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(de)?;
    Ok(value.split(',').map(String::from).collect::<Vec<_>>())
}

pub fn profiles(
    clients: web::Data<Clients>,
    web::Query(info): web::Query<ProfileRequest>,
) -> Result<impl Responder> {
    let mut profiles = Vec::with_capacity(info.names.len());

    for name in info.names {
        profiles.push(github::User::by_username(&name, &clients)?);
    }

    Ok(HttpResponse::Ok().json(profiles))
}
