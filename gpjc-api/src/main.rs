// #[macro_use]
extern crate actix_web;

use std::env;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

mod constants;
mod db;
mod handler;
mod types;

fn set_env() {
    let binding = std::env::current_dir().unwrap();
    let current_dir_path = binding.to_str().unwrap();
    let pdir = current_dir_path.to_owned().clone() + "/private-join-and-compute";
    std::env::set_current_dir(pdir).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    set_env();

    match db::execute_query(db::Query::CreateTable, Vec::new()) {
        Ok(_) => println!("Table created"),
        Err(err) => println!("Error {err}"),
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(handler::start_client_process)
            .service(handler::start_server_process)
            .service(handler::get_proof)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
