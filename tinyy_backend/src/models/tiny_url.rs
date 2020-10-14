use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::{DatabaseErrorKind, Error};
use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::Rng;
use validator::{Validate, ValidationError};
use validator_derive::Validate;

use crate::errors::TinyUrlError;
use crate::schema::tiny_urls;


#[derive(Debug, Insertable, Serialize, Queryable)]
pub struct TinyUrl {
    pub code: String,
    pub url: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewTinyUrl {
    pub code: Option<String>,
    #[validate(url, custom = "validate_http_url")]
    pub url: String
}

impl TinyUrl {

    pub fn new(new_tiny_url: NewTinyUrl, conn: &PgConnection) -> Result<TinyUrl, TinyUrlError> {
        new_tiny_url.validate().map_err(|_| TinyUrlError::InvalidHttpUrl)?;

        let mut code;

        if let None = new_tiny_url.code {
            loop {
                code = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .collect::<String>();

                match Self::get(&code, conn) {
                    Err(error) => {
                        if let TinyUrlError::CodeNotFound = error {
                            break;
                        } else {
                            return Err(error);
                        }
                    },
                    Ok(_) => continue
                }
            }
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

    pub fn get(code: &str, conn: &PgConnection) -> Result<TinyUrl, TinyUrlError> {
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


fn validate_http_url(url: &str) -> Result<(), ValidationError> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_http_url"))
    }
}
