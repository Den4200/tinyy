use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use validator_derive::Validate;

use crate::errors::UserError;
use crate::schema::users;

#[table_name = "users"]
#[derive(Deserialize, Serialize, Queryable, Insertable, Validate)]
pub struct User {
    pub id: Option<i32>,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String
}

impl User {
    pub fn new(user: User, conn: &PgConnection) -> Result<User, UserError> {
        user
            .validate()
            .map_err(|_| => UserError::InvalidUser)?;
        
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
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
