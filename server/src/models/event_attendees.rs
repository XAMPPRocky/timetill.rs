use diesel::*;
use snafu::{OptionExt, ResultExt};

use crate::{error, schema::*, Result};

#[derive(Debug, Queryable)]
pub struct EventAttendee {
    pub attendee_id: i32,
    pub event_id: i32,
    pub attend_date: chrono::DateTime<chrono::Utc>,
    pub recurring_attendee: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "event_attendees"]
pub struct NewEventAttendee {
    pub attendee_id: i32,
    pub event_id: i32,
    pub attend_date: chrono::DateTime<chrono::Utc>,
}

impl EventAttendee {
    pub fn attend(event: &super::Event, attendee_id: i32, conn: &PgConnection) -> Result<Self> {
        let event_id = event.event_id;

        let attend_date = event.next_date().context(error::Logic {
            detail: "Couldn't attend an event scheduled in the past.",
        })?;

        insert_into(event_attendees::table)
            .values(&NewEventAttendee {
                attendee_id,
                event_id,
                attend_date,
            })
            .get_result(conn)
            .context(error::Database)
    }
}
