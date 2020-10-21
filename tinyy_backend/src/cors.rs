use std::io::Cursor;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, HeaderMap, Method, Status};

pub struct Cors();

impl Cors {
    fn get_header(headers: &HeaderMap, name: &str) -> String {
        match headers.get_one(name) {
            Some(header) => header.to_string(),
            _ => "".to_string()
        }
    }
}

impl Fairing for Cors {

    fn info(&self) -> Info {
        Info {
            name: "Cors",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let req_headers = request.headers();

        let req_allow_origin = Self::get_header(req_headers, "Origin");
        response.set_header(Header::new("Allow-Control-Allow-Origin", req_allow_origin));

        if request.method() == Method::Options {
            let req_allow_headers = Cors::get_header(req_headers, "Access-Control-Request-Headers");
            let req_allow_method = Cors::get_header(req_headers, "Access-Control-Request-Method");

            response.set_header(Header::new("Access-Control-Allow-Headers", req_allow_headers));
            response.set_header(Header::new("Access-Control-Allow-Methods", req_allow_method));

            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
