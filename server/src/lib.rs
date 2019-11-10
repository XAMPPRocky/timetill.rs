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

use r2d2_redis::{r2d2, RedisConnectionManager};

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

trait ChronoExt {
    fn is_in_future(&self) -> bool;
}

impl ChronoExt for chrono::DateTime<chrono::Utc> {
    fn is_in_future(&self) -> bool {
        let now = chrono::Utc::now();

        let result = now.signed_duration_since(*self);

        result.is_zero()
    }
}
