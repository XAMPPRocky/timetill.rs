use actix_session::Session;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use snafu::{OptionExt, ResultExt};

use crate::{error, github};

pub struct CurrentUser(pub crate::github::User);

impl FromRequest for CurrentUser {
    type Error = crate::Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            // Convert to string and skip `Bearer/token` prefix.
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.split_whitespace().nth(1))
            .context(error::MissingAuthorisation)?;

        let clients = req.get_app_data::<crate::Clients>().context(error::Logic {
            detail: "No App Data available.",
        })?;

        let session = Session::extract(req).context(error::Actix)?;
        let user = github::User::current(token, &*clients, &session)?;

        Ok(CurrentUser(user))
    }
}
