use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("HTTP Error: {}\n{}", source, backtrace))]
    Actix {
        source: actix_web::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Database Error: {}\n{}", source, backtrace))]
    Database {
        source: diesel::result::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Reqwest Error: {}\n{}", source, backtrace))]
    HttpClient {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("JSON Encoding Error: {}\n{}", source, backtrace))]
    Json {
        source: serde_json::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Logic Error: {}\n{}", detail, backtrace))]
    Logic {
        detail: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Missing required Authorization header."))]
    MissingAuthorisation,

    #[snafu(display("R2D2 Error: {}\n{}", source, backtrace))]
    R2d2 {
        source: r2d2::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Redis Error: {}\n{}", source, backtrace))]
    Redis {
        source: redis::RedisError,
        backtrace: Backtrace,
    },

    #[snafu(display("Invalid URL: {}\n{}", source, backtrace))]
    UrlDecoding {
        source: serde_urlencoded::de::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Invalid URL: {}\n{}", source, backtrace))]
    UrlEncoding {
        source: serde_urlencoded::ser::Error,
        backtrace: Backtrace,
    },
}

impl actix_web::ResponseError for Error {}

impl From<diesel::result::Error> for Error {
    fn from(source: diesel::result::Error) -> Self {
        Self::Database {
            source,
            backtrace: Backtrace::new(),
        }
    }
}
