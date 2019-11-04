use actix_session::Session;
use actix_web::{http, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{error, github, models};

const AUTH_SESSION_STATE: &str = "state";

/// Query parameters for `/login`.
#[derive(Deserialize)]
pub struct LoginRequest {
    next_page: Option<String>,
}

/// Authenication session state.
#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct AuthState {
    id: String,
    next_page: Option<String>,
}

pub fn login(
    web::Query(info): web::Query<LoginRequest>,
    session: Session,
) -> crate::Result<impl Responder> {
    let id = uuid::Uuid::new_v4().to_string();
    let auth_state = AuthState {
        id,
        next_page: info.next_page,
    };

    session
        .set(AUTH_SESSION_STATE, &auth_state)
        .context(error::Actix)?;

    let parameters = serde_urlencoded::to_string(auth_state).context(error::UrlEncoding)?;

    let url = format!(
        "https://github.com/login/oauth/authorize?{}",
        serde_urlencoded::to_string(&[
            ("client_id", &**crate::env::GITHUB_ID),
            ("redirect_uri", &**crate::env::REDIRECT_URI),
            ("state", &*parameters),
        ])
        .context(error::UrlEncoding)?
    );

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish())
}

/// Authenication query parameters from GitHub.
#[derive(Deserialize)]
pub struct AuthRequest {
    state: String,
    code: String,
}

pub fn authorise(
    session: Session,
    clients: web::Data<crate::Clients>,
    web::Query(info): web::Query<AuthRequest>,
) -> crate::Result<impl Responder> {
    let state = session
        .get::<AuthState>(AUTH_SESSION_STATE)
        .context(error::Actix)?
        .and_then(|session_state| {
            serde_urlencoded::from_str::<AuthState>(&info.state)
                .ok()
                .filter(|query| query == &session_state)
        });

    if state.is_none() {
        session.clear();
        return crate::error::Logic {
            detail: String::from("Invalid State."),
        }
        .fail();
    }

    let state = state.unwrap();
    let url = format!(
        "https://github.com/login/oauth/access_token?{}",
        serde_urlencoded::to_string(&[
            ("client_id", &**crate::env::GITHUB_ID),
            ("client_secret", &**crate::env::GITHUB_SECRET),
            ("code", &*info.code),
            ("redirect_uri", &**crate::env::REDIRECT_URI),
            ("state", &*info.state),
        ])
        .context(error::UrlEncoding)?
    );

    #[derive(Deserialize)]
    struct GitHubAuth {
        access_token: String,
    }

    log::info!("Requesting access token");
    let body: String = clients
        .http
        .post(&url)
        .send()
        .context(error::HttpClient)?
        .text()
        .context(error::HttpClient)?;
    let token = serde_urlencoded::from_str::<GitHubAuth>(&body).context(error::UrlDecoding)?;

    // Always get a fresh instance of the data
    session.clear();
    let conn = clients.pg.get().context(error::R2d2)?;
    let user = github::User::current(&token.access_token, &clients, &session)?;
    models::User::insert(user.id, &user.login, &conn)?;

    Ok(HttpResponse::Found()
        .header(
            http::header::LOCATION,
            format!(
                "http://{}/authorise?{}",
                &**crate::env::CLIENT_HOSTNAME,
                serde_urlencoded::to_string(&[
                    ("access_token", token.access_token),
                    ("next_page", state.next_page.unwrap_or_else(String::new))
                ])
                .context(error::UrlEncoding)?
            ),
        )
        .finish())
}
