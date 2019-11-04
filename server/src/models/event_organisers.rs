use diesel::*;
use snafu::ResultExt;

use crate::{error, schema::*, Result};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "event_organisers"]
pub struct EventOrganiser {
    pub organiser_id: i32,
    pub event_id: i32,
}

impl EventOrganiser {
    pub fn by_event(event_id: i32, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        event_organisers::table
            .filter(event_organisers::event_id.eq(event_id))
            .get_results(conn)
    }

    pub fn insert(organiser_id: i32, event_id: i32, conn: &PgConnection) -> Result<Self> {
        users::table
            .find(organiser_id)
            .execute(conn)
            .context(error::Database)?;

        events::table
            .find(event_id)
            .execute(conn)
            .context(error::Database)?;

        let organiser = Self {
            organiser_id,
            event_id,
        };

        insert_into(event_organisers::table)
            .values(&organiser)
            .get_result(conn)
            .context(error::Database)
    }
}
