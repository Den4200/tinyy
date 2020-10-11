use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::db;
use crate::models::tiny_url::TinyUrl;


#[post("/new", format = "json", data = "<tiny_url>")]
pub fn new_tiny_url(tiny_url: Json<TinyUrl>, conn: db::Conn) -> Json<TinyUrl> {
    let tiny_url = TinyUrl { ..tiny_url.into_inner() };
    Json(TinyUrl::new(tiny_url, &conn))
}

#[get("/raw/<code>")]
pub fn get_tiny_link(code: String, conn: db::Conn) -> Json<TinyUrl> {
    let tiny_url = TinyUrl::get(code, &conn);
    Json(tiny_url)
}

#[get("/<code>")]
pub fn redirect_tiny_link(code: String, conn: db::Conn) -> Redirect {
    let tiny_url = TinyUrl::get(code, &conn);
    Redirect::to(tiny_url.url)
}
