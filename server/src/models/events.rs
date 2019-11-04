use diesel::*;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
    error, github,
    models::{EventOrganiser, User},
    schema::{event_attendees, event_organisers, events, users},
    Result,
};

#[derive(Serialize, Queryable)]
pub struct Event {
    pub event_id: i32,
    pub about_md: String,
    pub about_html: String,
    pub address: String,
    pub approved: bool,
    pub city: String,
    pub country: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub event: String,
    pub event_date: chrono::DateTime<chrono::Utc>,
    pub event_duration_amount: i16,
    pub event_duration_step: String,
    pub gps: Option<diesel_geometry::data_types::PgPoint>,
    pub occurance_amount: Option<i16>,
    pub occurance_step: Option<String>,
    pub region: String,
    pub slug: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub about_md: String,
    #[serde(default)]
    pub about_html: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub event: String,
    pub event_duration_step: String,
    pub event_duration_amount: i16,
    pub event_date: chrono::DateTime<chrono::Utc>,
    pub gps: Option<diesel_geometry::data_types::PgPoint>,
    pub occurance_step: Option<String>,
    pub occurance_amount: Option<i16>,
    pub region: String,
    pub slug: String,
}

impl Event {
    pub fn all(conn: &PgConnection) -> Result<Vec<Self>> {
        events::table.get_results(conn).context(error::Database)
    }

    pub fn by_url(slug: &str, conn: &PgConnection) -> Result<Self> {
        events::table
            .filter(events::slug.eq(slug))
            .get_result(conn)
            .context(error::Database)
    }

    pub fn insert(new_event: &NewEvent, conn: &PgConnection) -> Result<Self> {
        insert_into(events::table)
            .values(new_event)
            .get_result(conn)
            .context(error::Database)
    }
}
