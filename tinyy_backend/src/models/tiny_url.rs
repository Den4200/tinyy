use rand::Rng;
use rand::distributions::Alphanumeric;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};

use crate::schema::tiny_urls;


#[derive(Debug, Insertable, Serialize, Queryable)]
pub struct TinyUrl {
    pub code: String,
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct NewTinyUrl {
    pub code: Option<String>,
    pub url: String
}

impl TinyUrl {

    pub fn new(new_tiny_url: NewTinyUrl, conn: &PgConnection) -> TinyUrl {
        let code;

        if let None = new_tiny_url.code {
            code = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .collect::<String>();
        } else {
            code = new_tiny_url.code.unwrap();
        }

        let tiny_url = TinyUrl {
            code,
            url: new_tiny_url.url
        };

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
