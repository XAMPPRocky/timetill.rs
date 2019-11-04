use actix_web::{web, HttpResponse, Responder};
use diesel::*;
use serde::Serialize;
use snafu::ResultExt;

use crate::{error, github, middleware, models, Result};

#[derive(Serialize)]
struct EventResponse {
    events: Vec<Event>,
}

impl EventResponse {
    fn new(events: Vec<Event>) -> Self {
        Self { events }
    }
}

/// An Event to return from the API. The distniction between this an
/// `models::Event` is that this uses Github user's instead of timetill.rs users.
#[derive(Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub details: models::Event,
    pub organisers: Vec<github::User>,
    pub attendees: Vec<github::User>,
}

impl Event {
    pub fn from_model(
        details: models::Event,
        conn: &PgConnection,
        clients: &crate::Clients,
    ) -> Result<Self> {
        let organisers = models::User::organisers_by_event(details.event_id, &conn)?
            .into_iter()
            .map(|u| github::User::from_model(u, clients))
            .collect::<Result<_>>()?;
        let attendees = models::User::attendees_by_event(details.event_id, &conn)?
            .into_iter()
            .map(|u| github::User::from_model(u, clients))
            .collect::<Result<_>>()?;

        Ok(Self {
            attendees,
            details,
            organisers,
        })
    }

    pub fn all(clients: &crate::Clients) -> Result<Vec<Self>> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::all(&conn)?
            .into_iter()
            .map(|e| Self::from_model(e, &conn, clients))
            .collect()
    }

    pub fn by_url(slug: &str, clients: &crate::Clients) -> Result<Self> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::by_url(slug, &conn).and_then(|e| Self::from_model(e, &conn, clients))
    }

    pub fn insert(
        new_event: &models::NewEvent,
        organiser: i32,
        clients: &crate::Clients,
    ) -> Result<Self> {
        let conn = clients.pg.get().context(error::R2d2)?;

        conn.transaction::<_, crate::Error, _>(|| {
            let event = models::Event::insert(new_event, &conn)?;
            let organiser = models::EventOrganiser::insert(organiser, event.event_id, &conn)?;
            let user = models::User::find(organiser.organiser_id, &conn)?;
            let gh_user = github::User::by_username(&user.github_name, clients)?;

            Ok(Self {
                details: event,
                organisers: vec![gh_user],
                attendees: Vec::new(),
            })
        })
    }
}

pub fn create(
    middleware::CurrentUser(user): middleware::CurrentUser,
    clients: web::Data<crate::Clients>,
    mut form: web::Json<models::NewEvent>,
) -> Result<impl Responder> {
    form.about_html = {
        let parser = pulldown_cmark::Parser::new(&form.about_md);
        let mut output = String::new();
        pulldown_cmark::html::push_html(&mut output, parser);
        output
    };

    let new_event = Event::insert(&form, user.id, &clients)?;

    Ok(HttpResponse::Ok().json(new_event))
}

pub fn get(
    clients: web::Data<crate::Clients>,
    input_slug: web::Path<String>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(Event::by_url(&*input_slug, &clients)?))
}

pub fn list(clients: web::Data<crate::Clients>) -> Result<impl Responder> {
    let events = Event::all(&clients)?;
    Ok(HttpResponse::Ok().json(EventResponse::new(events)))
}
