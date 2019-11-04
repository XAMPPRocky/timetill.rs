use diesel::*;
use serde::Serialize;
use snafu::ResultExt;

use crate::{
    error,
    schema::{event_attendees, event_organisers, users},
    Result,
};

#[derive(Queryable, Serialize)]
pub struct User {
    pub github_id: i32,
    pub github_name: String,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub reviewer: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub trusted: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    github_id: i32,
    github_name: &'a str,
}

fn bool_is_false(b: &bool) -> bool {
    !b
}

impl User {
    pub fn find(github_id: i32, conn: &PgConnection) -> Result<Self> {
        users::table
            .find(github_id)
            .get_result(conn)
            .context(error::Database)
    }

    pub fn organisers_by_event(event_id: i32, conn: &PgConnection) -> Result<Vec<Self>> {
        users::table
            .left_join(event_organisers::table)
            .filter(event_organisers::event_id.eq(event_id))
            .select(users::all_columns)
            .get_results::<Self>(conn)
            .context(error::Database)
    }

    pub fn attendees_by_event(event_id: i32, conn: &PgConnection) -> Result<Vec<Self>> {
        users::table
            .left_join(event_attendees::table)
            .filter(event_attendees::event_id.eq(event_id))
            .select(users::all_columns)
            .get_results::<Self>(conn)
            .context(error::Database)
    }

    pub fn insert(github_id: i32, github_name: &str, conn: &PgConnection) -> Result<Self> {
        insert_into(users::table)
            .values(&NewUser {
                github_id,
                github_name,
            })
            .get_result(conn)
            .context(error::Database)
    }
}
