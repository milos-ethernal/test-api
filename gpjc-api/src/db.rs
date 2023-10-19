use odbc::{
    create_environment_v3,
    odbc_safe::AutocommitOn,
    Connection, DiagnosticRecord,
    ResultSetState::{Data, NoData},
    Statement,
};

use crate::types::LogEntry;

pub enum Query {
    CreateTable,
    InsertLog,
    UpdateLog,
    GetLog,
}

/// # CreateTable
/// No params
/// # InsertLog
/// TransactionId, IsInitiator = (0, 1)
/// # UpdateLog
/// Result, Proof, TransactionId
/// # GetLog
/// TransactionId
pub fn execute_query(
    query: Query,
    params: Vec<String>,
) -> Result<Option<LogEntry>, DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let connection_string = "Driver={ODBC Driver 18 for SQL Server};\
    Server=0.0.0.0;\
    UID=SA;\
    PWD=Ethernal!123;TrustServerCertificate=Yes;Database=gpjc_data;\
    ";

    let conn = env.connect_with_connection_string(&connection_string)?;
    execute_statement(&conn, query, params)
}

fn execute_statement<'env>(
    conn: &Connection<'env, AutocommitOn>,
    query: Query,
    params: Vec<String>,
) -> Result<Option<LogEntry>, DiagnosticRecord> {
    let stmt = Statement::with_parent(conn)?;

    match query {
        Query::CreateTable => {
            let sql_text = "IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'gpjc_logs')
                                    CREATE TABLE gpjc_logs (
                                        TRANSACTION_ID INTEGER PRIMARY KEY,
                                        RESULT INTEGER,
                                        COMPUTATION_START DATETIME,
                                        COMPUTATION_END DATETIME,
                                        PROOF INTEGER,
                                        IS_INITIATOR BIT
                                    )";
            stmt.exec_direct(&sql_text)?;
        }
        Query::InsertLog => {
            if params.len() != 2 {
                return Err(DiagnosticRecord::empty());
            }
            let sql_text = format!("INSERT INTO gpjc_logs (TRANSACTION_ID, RESULT, COMPUTATION_START, IS_INITIATOR) VALUES ({}, {}, {}, {})", params[0], "-1", "GETUTCDATE()".to_string(), params[1]);
            stmt.exec_direct(&sql_text)?;
        }
        Query::UpdateLog => {
            if params.len() != 3 {
                return Err(DiagnosticRecord::empty());
            }
            let sql_text = format!("UPDATE gpjc_logs SET RESULT = {}, COMPUTATION_END = {}, PROOF = {} WHERE TRANSACTION_ID = {}",params[0], "GETUTCDATE()".to_string(), params[1], params[2]);
            stmt.exec_direct(&sql_text)?;
        }
        Query::GetLog => {
            if params.len() != 1 {
                return Err(DiagnosticRecord::empty());
            }
            let sql_text = format!(
                "SELECT * FROM gpjc_logs WHERE TRANSACTION_ID = {}",
                params[0]
            );
            match stmt.exec_direct(&sql_text)? {
                Data(mut stmt) => {
                    let cols = stmt.num_result_cols()?;
                    while let Some(mut cursor) = stmt.fetch()? {
                        let mut params: Vec<String> = vec![];
                        for i in 1..(cols + 1) {
                            match cursor.get_data::<&str>(i as u16)? {
                                Some(val) => params.push(val.to_owned()),
                                None => params.push("NULL".to_string()),
                            }
                        }

                        return Ok(Some(LogEntry {
                            transaction_id: params[0].to_owned(),
                            result: params[1].to_owned(),
                            computation_start: params[2].to_owned(),
                            computation_end: params[3].to_owned(),
                            is_initiator: params[4].to_owned(),
                        }));
                    }
                }
                NoData(_) => return Ok(None),
            }
        }
    }

    Ok(None)
}
