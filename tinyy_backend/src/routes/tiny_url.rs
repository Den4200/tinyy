use rocket::response::Redirect;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::db;
use crate::errors::TinyUrlError;
use crate::models::tiny_url::{NewTinyUrl, TinyUrl};


#[post("/", format = "json", data = "<tiny_url>")]
pub fn new_tiny_url(tiny_url: Json<NewTinyUrl>, conn: db::Conn) -> Result<Json<TinyUrl>, Status> {
    let new_tiny_url = NewTinyUrl { ..tiny_url.into_inner() };
    let tiny_url = TinyUrl::new(new_tiny_url, &conn)
        .map_err(|err| {
            if let TinyUrlError::UniqueCodeViolation = err {
                Status::Conflict
            } else {
                Status::InternalServerError
            }
        })?;

    Ok(Json(tiny_url))
}

#[get("/<code>/raw")]
pub fn get_tiny_link(code: String, conn: db::Conn) -> Result<Json<TinyUrl>, Status> {
    let tiny_url = TinyUrl::get(code, &conn)
        .map_err(|err| {
            if let TinyUrlError::CodeNotFound = err {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })?;

    Ok(Json(tiny_url))
}

#[get("/<code>")]
pub fn redirect_tiny_link(code: String, conn: db::Conn) -> Result<Redirect, Status> {
    let tiny_url = TinyUrl::get(code, &conn)
        .map_err(|err| {
            if let TinyUrlError::CodeNotFound = err {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })?;

    Ok(Redirect::to(tiny_url.url))
}
