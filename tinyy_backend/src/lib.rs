#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;

use rocket::http::Method;
use rocket::Rocket;
use rocket::fairing::AdHoc;

use rocket_cors::{
    AllowedOrigins,
    Cors,
    CorsOptions
};

mod db;
mod config;
mod errors;
mod models;
mod routes;
mod schema;


embed_migrations!();


fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = db::Conn::get_one(&rocket).expect("Failed to get database connection..");

    match embedded_migrations::run_with_output(&*conn, &mut std::io::stdout()) {
        Ok(()) => {
            println!("Successfully ran database migrations.");
            Ok(rocket)
        },
        Err(e) => {
            println!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}


fn create_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(
        &[
            "https://tinyy.io",
            "http://localhost:3000",
            "http://127.0.0.1:3000"
        ]
    );

    let allowed_methods = vec![Method::Get, Method::Post]
        .into_iter()
        .map(From::from)
        .collect();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS..");

    cors
}


pub fn rocket() -> Rocket {
    dotenv().ok();

    rocket::custom(config::from_env())
        .mount(
            "/",
            routes![
                routes::tiny_url::get_tiny_link,
                routes::tiny_url::new_tiny_url,
                routes::tiny_url::redirect_tiny_link
            ]
        )
        .attach(create_cors())
        .attach(db::Conn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_migrations))
}
