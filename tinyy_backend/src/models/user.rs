use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::errors::UserError;
use crate::schema::users;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[table_name = "users"]
pub struct NewUser {
    pub id: Option<i32>,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String
}

impl User {
    pub fn new(new_user: NewUser, conn: &PgConnection) -> Result<User, UserError> {
        new_user
            .validate()
            .map_err(|_| UserError::InvalidUser)?;
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|error| {
                if let Error::DatabaseError(kind, _) = error {
                    if let DatabaseErrorKind::UniqueViolation = kind {
                        return UserError::DuplicateEmail;
                    }
                }
                UserError::GenericServerError
            })
    }
}
