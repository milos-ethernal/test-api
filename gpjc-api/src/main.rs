// #[macro_use]
extern crate actix_web;

use std::env;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use db::create_table_and_pool;

mod api;
mod constants;
mod db;
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

    // connect to SQLite DB
    //let manager = SqliteConnectionManager::file("gpjc_logs.db");
    let pool = create_table_and_pool();

    set_env();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(api::start_client_process)
            .service(api::start_server_process)
            .service(api::get_transactions)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
