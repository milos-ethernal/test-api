use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::str::FromStr;

use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};

use crate::constants::APPLICATION_JSON;
use crate::db;
use crate::types::{ClientStartRequest, ProofRequest, Response, ServerStartRequest};

fn get_path(file_name: &str) -> String {
    let binding = std::env::current_dir().unwrap();
    let current_dir_path = binding.to_str().unwrap();
    let sanctions_dir_path =
        current_dir_path.replace("private-join-and-compute", "") + "sanction-lists/";

    let paths = fs::read_dir(sanctions_dir_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    paths
        .iter()
        .find(|&x| x.contains(file_name))
        .unwrap()
        .to_string()
}

pub fn start_client(_transaction_id: i32, _destination_address: String) -> Response {
    let client_csv_path = get_path("UN_test.csv");

    let output = Command::new("bazel-bin/private_join_and_compute/client")
        .arg(format!("--client_data_file={client_csv_path}",))
        //.arg(format!(" --port={destination_address}"))
        .output()
        .unwrap();

    // TODO: Remove comments for client side DB
    #[cfg(feature = "client")]
    {
        let params = vec![transaction_id.to_string(), "1".to_string()];
        match db::execute_query(db::Query::InsertLog, params) {
            Ok(_) => println!("Entered log in db"),
            Err(err) => println!("Insert into gpjc_logs failed with error: {err}"),
        };
    }
    match output.status.code() {
        Some(code) => {
            if code == 0 {
                let output_text = String::from_utf8_lossy(&output.stdout).into_owned();
                #[cfg(feature = "client")]
                {
                    let sliced_text: Vec<&str> = output_text.split(' ').collect();

                    let params = vec![
                        sliced_text[5].to_string(),
                        "PROOF".to_string(), // TODO: Update GPJC for proof parsing
                        transaction_id.to_string(),
                    ];
                    match db::execute_query(db::Query::UpdateLog, params) {
                        Ok(_) => println!("Updated log in db"),
                        Err(err) => println!("Insert into gpjc_logs failed with error: {err}"),
                    };
                }

                return Response {
                    exit_code: code,
                    data: output_text,
                };
            } else {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
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

pub fn start_server(transaction_id: String) -> Response {
    let params = vec![transaction_id.clone(), "0".to_string()];
    match db::execute_query(db::Query::InsertLog, params) {
        Ok(_) => println!("Entered log in db"),
        Err(err) => println!("Insert into gpjc_logs failed with error: {err}"),
    };

    let server_csv_path = get_path("UN_List.csv");

    let output = Command::new("bazel-bin/private_join_and_compute/server")
        .arg(format!("--server_data_file={}", server_csv_path))
        .output()
        .unwrap();

    match output.status.code() {
        Some(code) => {
            if code == 0 {
                let output_text = String::from_utf8_lossy(&output.stdout).into_owned();

                let params = vec![
                    output_text.clone(),
                    "PROOF".to_string(), // TODO: Update GPJC for proof parsing
                    transaction_id,
                ];
                match db::execute_query(db::Query::UpdateLog, params) {
                    Ok(_) => println!("Updated log in db"),
                    Err(err) => println!("Insert into gpjc_logs failed with error: {err}"),
                };

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
pub async fn start_client_process(request_data: Json<ClientStartRequest>) -> impl Responder {
    let resp = start_client(
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
pub async fn start_server_process(request_data: Json<ServerStartRequest>) -> impl Responder {
    tokio::spawn(async move {
        let resp = start_server(request_data.tx_id.clone());
        if resp.exit_code != 0 {
            println!("ERROR: GPJC failed with error: {}", resp.data);
            return;
        }

        let mut map = HashMap::new();
        map.insert("TransactionId", request_data.tx_id.clone());
        map.insert("Value", resp.data);

        let client = reqwest::Client::new();
        let _res = client
            .post("http://localhost:4000/submitTransactionProof")
            .json(&map)
            .send()
            .await;
    });

    return HttpResponse::Ok();
}

#[get("/proof")]
pub async fn get_proof(request_data: Json<ProofRequest>) -> impl Responder {
    let params = vec![FromStr::from_str(request_data.tx_id.as_str()).unwrap()];
    match db::execute_query(db::Query::GetLog, params) {
        Ok(val) => match val {
            Some(resp) => return HttpResponse::Ok().content_type(APPLICATION_JSON).json(resp),
            None => {
                return HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json("Log with this transaction id does not exist")
            }
        },
        Err(_err) => {
            return HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON)
                .json("")
        }
    };
}
