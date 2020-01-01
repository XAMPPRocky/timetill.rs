use actix_web::http;
use redis::Commands;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{error, github, models, Result};

const USER_ETAG: &str = "etag";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: i32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: String,
    pub location: String,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: String,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub private_gists: Option<i32>,
    pub total_private_repos: Option<i32>,
    pub owned_private_repos: Option<i32>,
    pub disk_usage: Option<i32>,
    pub collaborators: Option<i32>,
    pub two_factor_authentication: Option<bool>,

    /// Extra fields from `models::User`
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub model: Option<models::User>,
}

impl User {
    /// Get Github user based on the request given, and cache the result based
    /// on `etag` if available. If the request does not receive a cached
    /// response `on_new` is called with the user and its etag.
    fn get<F>(
        clients: &crate::Clients,
        mut request: reqwest::RequestBuilder,
        etag: Option<&String>,
        mut on_new: F,
    ) -> Result<Self>
    where
        F: FnMut(&github::User, &str) -> Result<()>,
    {
        let mut cache = clients.redis.get().context(error::R2d2)?;

        if let Some(etag) = etag {
            request = request.header(http::header::IF_NONE_MATCH, etag);
        }

        let mut response = request.send().context(error::HttpClient)?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let etag = response
                    .headers()
                    .get(http::header::ETAG)
                    .and_then(|hv| hv.to_str().ok().map(String::from))
                    .unwrap();

                let user = response
                    .json::<crate::github::User>()
                    .context(error::HttpClient)?;

                cache
                    .set(&etag, &serde_json::to_string(&user).unwrap())
                    .context(error::Redis)?;

                (on_new)(&user, &etag)?;

                Ok(user)
            }

            reqwest::StatusCode::NOT_MODIFIED => {
                let raw = cache
                    .get::<_, String>(etag.unwrap())
                    .context(error::Redis)?;

                serde_json::from_str(&raw).context(error::Json)
            }
            _ => error::Logic {
                detail: response.text().context(error::HttpClient)?,
            }
            .fail(),
        }
    }

    /// Fetches the user from GitHub. **Note** this will never include the
    /// `model` field.
    pub fn from_github(
        token: &str,
        clients: &crate::Clients,
        session: &actix_session::Session,
    ) -> Result<Self> {
        let request = clients
            .http
            .get("https://api.github.com/user")
            .header(http::header::AUTHORIZATION, format!("token {}", token));
        let etag = session.get::<String>(USER_ETAG).ok().and_then(|x| x);

        Self::get(&clients, request, etag.as_ref(), |_, etag| {
            session.set(USER_ETAG, etag).context(error::Actix)?;
            Ok(())
        })
    }

    pub fn current(
        token: &str,
        clients: &crate::Clients,
        session: &actix_session::Session,
    ) -> Result<Self> {
        let mut gh_user = Self::from_github(token, clients, session)?;
        let conn = clients.pg.get().context(error::R2d2)?;
        let model = models::User::find(gh_user.id, &conn)?;
        gh_user.model = Some(model);

        Ok(gh_user)
    }

    fn by_username(username: &str, clients: &crate::Clients) -> Result<Self> {
        let mut redis = clients.redis.get().context(error::R2d2)?;
        let key = format!("github-{}", username);
        let etag: Option<String> = redis.get(&key).context(error::Redis)?;

        let url = format!("https://api.github.com/users/{}", username);

        let request = clients.http.get(&url).query(&[
            ("client_id", &*crate::env::GITHUB_ID),
            ("client_secret", &*crate::env::GITHUB_SECRET),
        ]);

        Self::get(&clients, request, etag.as_ref(), |user, _| {
            redis
                .set(&key, serde_json::to_string(&user).unwrap())
                .context(error::Redis)?;
            Ok(())
        })
    }

    pub fn from_model(user: models::User, clients: &crate::Clients) -> Result<Self> {
        let mut gh_user = github::User::by_username(&user.github_name, clients)?;
        gh_user.model = Some(user);

        Ok(gh_user)
    }
}
