use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Response {
    pub exit_code: i32,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientStartRequest {
    pub tx_id: String,
    pub receiver: String,
    pub to: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerStartRequest {
    pub tx_id: String,
    pub policy_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub tx_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    pub transaction_id: String,
    pub result: String,
    pub computation_start: String,
    pub computation_end: String,
    pub is_initiator: String,
}
