use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::db;
use crate::models::tiny_url::{NewTinyUrl, TinyUrl};


#[post("/", format = "json", data = "<tiny_url>")]
pub fn new_tiny_url(tiny_url: Json<NewTinyUrl>, conn: db::Conn) -> Json<TinyUrl> {
    let tiny_url = NewTinyUrl { ..tiny_url.into_inner() };
    Json(TinyUrl::new(tiny_url, &conn))
}

#[get("/<code>/raw")]
pub fn get_tiny_link(code: String, conn: db::Conn) -> Json<TinyUrl> {
    let tiny_url = TinyUrl::get(code, &conn);
    Json(tiny_url)
}

#[get("/<code>")]
pub fn redirect_tiny_link(code: String, conn: db::Conn) -> Redirect {
    let tiny_url = TinyUrl::get(code, &conn);
    Redirect::to(tiny_url.url)
}
