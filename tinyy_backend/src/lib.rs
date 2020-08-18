#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;

mod config;
mod routes;

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();

    rocket::custom(config::from_env())
        .mount(
            "/",
            routes![
                routes::health::health
            ]
        )
}
