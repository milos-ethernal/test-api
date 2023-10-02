use std::process::Command;
use std::str::FromStr;

use actix_web::web::{self, Json};
use actix_web::{get, post, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::constants::{APPLICATION_JSON, CLIENT_CSV_PATH, SERVER_CSV_PATH};
use crate::db;
use crate::types::{ClientStartRequest, Response, ServerStartRequest};

pub fn start_client(
    _data: web::Data<Pool<SqliteConnectionManager>>,
    _transaction_id: i32,
    destination_address: String,
) -> Response {
    let output = Command::new("bazel-bin/private_join_and_compute/client")
        .arg(format!("--client_data_file={CLIENT_CSV_PATH}",))
        .arg(format!(" --port={destination_address}"))
        .output()
        .unwrap();

    // match db::insert_data(
    //     &data,
    //     transaction_id,
    //     -1,
    //     chrono::offset::Utc::now().to_string(),
    //     true,
    // ) {
    //     Ok(_) => println!("enterted log in db"),
    //     Err(err) => println!("error on inpit in db: {:#?}", err),
    // }

    match output.status.code() {
        Some(code) => {
            if code == 0 {
                let output_text = String::from_utf8_lossy(&output.stdout).into_owned();
                // let sliced_text: Vec<&str> = output_text.split(' ').collect();

                // match db::update_data(
                //     &data,
                //     transaction_id,
                //     FromStr::from_str(&sliced_text[5]).unwrap(),
                //     chrono::offset::Utc::now().to_string(),
                // ) {
                //     Ok(_) => println!("updated log in db"),
                //     Err(err) => println!("error on update in db: {:#?}", err),
                // }

                return Response {
                    exit_code: code,
                    data: output_text,
                };
            } else {
                return Response {
                    exit_code: code,
                    data: String::from_utf8_lossy(&output.stdout).into_owned(),
                };
            }
        }
        None => {
            return Response {
                exit_code: 1,
                data: "Error in client execution".to_string(),
            }
        }
    }
}

pub fn start_server(
    data: web::Data<Pool<SqliteConnectionManager>>,
    transaction_id: i32,
) -> Response {
    let output = Command::new("bazel-bin/private_join_and_compute/server")
        .arg(format!("--server_data_file={}", SERVER_CSV_PATH))
        .output()
        .unwrap();

    match db::insert_data(
        &data,
        transaction_id,
        -1,
        chrono::offset::Utc::now().to_string(),
        false,
    ) {
        Ok(_) => println!("enterted log in db"),
        Err(err) => println!("error on inpit in db: {:#?}", err),
    }

    match output.status.code() {
        Some(code) => {
            if code == 0 {
                let output_text = String::from_utf8_lossy(&output.stdout).into_owned();
                let sliced_text: Vec<&str> = output_text.split(' ').collect();

                match db::update_data(
                    &data,
                    transaction_id,
                    FromStr::from_str(&sliced_text[3][0..sliced_text[3].len() - 1]).unwrap(),
                    chrono::offset::Utc::now().to_string(),
                ) {
                    Ok(_) => println!("updated log in db"),
                    Err(err) => println!("error on update in db: {:#?}", err),
                }

                return Response {
                    exit_code: code,
                    data: output_text,
                };
            } else {
                return Response {
                    exit_code: code,
                    data: String::from_utf8_lossy(&output.stdout).into_owned(),
                };
            }
        }
        None => {
            return Response {
                exit_code: 1,
                data: "Error in server execution".to_string(),
            }
        }
    }
}

#[post("/start-client")]
pub async fn start_client_process(
    data: web::Data<Pool<SqliteConnectionManager>>,
    request_data: Json<ClientStartRequest>,
) -> impl Responder {
    let resp = start_client(
        data,
        FromStr::from_str(request_data.tx_id.as_str()).unwrap(),
        request_data.to.clone(),
    );

    if resp.exit_code == 0 {
        return HttpResponse::Ok().content_type(APPLICATION_JSON).json(resp);
    } else {
        return HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(resp);
    }
}

#[post("/start-server")]
pub async fn start_server_process(
    data: web::Data<Pool<SqliteConnectionManager>>,
    request_data: Json<ServerStartRequest>,
) -> impl Responder {
    let resp = start_server(
        data,
        FromStr::from_str(request_data.tx_id.as_str()).unwrap(),
    );

    if resp.exit_code == 0 {
        return HttpResponse::Ok().content_type(APPLICATION_JSON).json(resp);
    } else {
        return HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(resp);
    }
}

#[get("/transactions")]
pub async fn get_transactions(data: web::Data<Pool<SqliteConnectionManager>>) -> impl Responder {
    match db::select_all_data(&data) {
        Ok(log_entries) => {
            return HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(log_entries);
        }
        Err(err) => {
            println!("{:#?}", err);
            return HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON)
                .json("Error loading data from database");
        }
    }
}
