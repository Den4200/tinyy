use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};

use crate::schema::tiny_urls;


#[derive(Debug, Deserialize, Insertable, Serialize, Queryable)]
pub struct TinyUrl {
    pub code: String,
    pub url: String
}

impl TinyUrl {

    pub fn new(tiny_url: TinyUrl, conn: &PgConnection) -> TinyUrl {
        diesel::insert_into(tiny_urls::table)
            .values(&tiny_url)
            .get_result(conn)
            .expect("Error creating new tiny url")
    }

    pub fn get(code: String, conn: &PgConnection) -> TinyUrl {
        tiny_urls::table
            .find(code)
            .first::<TinyUrl>(conn)
            .expect("Tiny url was not found")
    }
}
