use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::{DatabaseErrorKind, Error};
use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::errors::TinyUrlError;
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

    pub fn new(new_tiny_url: NewTinyUrl, conn: &PgConnection) -> Result<TinyUrl, TinyUrlError> {
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
            .map_err(|error| {
                if let Error::DatabaseError(kind, _) = error {
                    if let DatabaseErrorKind::UniqueViolation = kind {
                        return TinyUrlError::UniqueCodeViolation;
                    } 
                }
                TinyUrlError::GenericServerError
            })
    }

    pub fn get(code: String, conn: &PgConnection) -> Result<TinyUrl, TinyUrlError> {
        tiny_urls::table
            .find(code)
            .first::<TinyUrl>(conn)
            .map_err(|error| {
                if let Error::NotFound = error {
                    TinyUrlError::CodeNotFound
                } else {
                    TinyUrlError::GenericServerError
                }
            })
    }
}
