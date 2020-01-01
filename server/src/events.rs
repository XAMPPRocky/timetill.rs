use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Responder};
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
#[derive(Debug, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub details: models::Event,
    pub organisers: Vec<github::User>,
    pub attendees: Vec<github::User>,
}

impl Event {
    /// Map from the database model's users to GitHub users.
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

    /// Retrieve all approved events.
    pub fn approved(clients: &crate::Clients) -> Result<Vec<Self>> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::approved(&conn)?
            .into_iter()
            .map(|e| Self::from_model(e, &conn, clients))
            .collect()
    }

    /// Retrieve all unapproved events.
    pub fn unapproved(clients: &crate::Clients) -> Result<Vec<Self>> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::unapproved(&conn)?
            .into_iter()
            .map(|e| Self::from_model(e, &conn, clients))
            .collect()
    }

    /// Retrieve all events.
    pub fn attend(slug: &str, user: &github::User, clients: &crate::Clients) -> Result<Self> {
        let conn = clients.pg.get().context(error::R2d2)?;
        let event = models::Event::by_url(slug, &conn)?;
        let user = models::User::find(user.id, &conn)?;

        models::EventAttendee::attend(&event, user.github_id, &conn)?;

        Self::from_model(event, &conn, clients)
    }

    /// Retrieve event by it's unique URL `slug`.
    pub fn by_url(slug: &str, clients: &crate::Clients) -> Result<Self> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::by_url(slug, &conn).and_then(|e| Self::from_model(e, &conn, clients))
    }

    /// Approve an event. Requires a user to have the `reviewer` permission.
    pub fn approve(slug: &str, clients: &crate::Clients) -> Result<Self> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::approve(slug, &conn).and_then(|e| Self::from_model(e, &conn, clients))
    }

    /// Approve an event. Requires a user to have the `reviewer` permission.
    pub fn deny(slug: &str, clients: &crate::Clients) -> Result<()> {
        let conn = clients.pg.get().context(error::R2d2)?;
        models::Event::deny(slug, &conn)
    }

    /// Insert `new_event` into `events` and assign `organiser` as the first
    /// event organiser.
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
            let gh_user = github::User::from_model(user, clients)?;

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
    req: HttpRequest,
    clients: web::Data<crate::Clients>,
    slug: web::Path<String>,
) -> Result<impl Responder> {
    let event = Event::by_url(&*slug, &clients)?;

    if event.details.approved {
        Ok(HttpResponse::Ok().json(event))
    } else {
        match middleware::CurrentUser::extract(&req) {
            Ok(middleware::CurrentUser(user))
                if user.model.as_ref().map(|m| m.reviewer).unwrap_or(false)
                    || event.organisers.iter().any(|o| o.id == user.id) =>
            {
                Ok(HttpResponse::Ok().json(event))
            }
            _ => error::NotFound.fail(),
        }
    }
}

pub fn list(clients: web::Data<crate::Clients>) -> Result<impl Responder> {
    let events = Event::approved(&clients)?;
    Ok(HttpResponse::Ok().json(EventResponse::new(events)))
}

pub fn review_queue(
    clients: web::Data<crate::Clients>,
    _reviewer: middleware::Reviewer,
) -> Result<impl Responder> {
    let events = Event::unapproved(&clients)?;
    Ok(HttpResponse::Ok().json(EventResponse::new(events)))
}

pub fn approve(
    clients: web::Data<crate::Clients>,
    _reviewer: middleware::Reviewer,
    slug: web::Path<String>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(Event::approve(&slug, &clients)?))
}

pub fn deny(
    clients: web::Data<crate::Clients>,
    _reviewer: middleware::Reviewer,
    slug: web::Path<String>,
) -> Result<impl Responder> {
    Event::deny(&slug, &clients)?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn attend(
    slug: web::Path<String>,
    middleware::CurrentUser(user): middleware::CurrentUser,
    clients: web::Data<crate::Clients>,
) -> Result<impl Responder> {
    let event = Event::attend(&slug, &user, &clients)?;

    Ok(HttpResponse::Ok().json(event))
}
