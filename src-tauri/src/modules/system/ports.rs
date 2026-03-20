use serde::{Deserialize, Serialize};

use crate::core::response::ApiResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub port: u16,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillPortResult {
    pub attempted: bool,
    pub port: u16,
    pub note: String,
}

pub fn list_ports() -> ApiResponse<Vec<PortInfo>> {
    ApiResponse::success(
        "Placeholder implementation: active-port scan is pending.",
        Vec::new(),
    )
}

pub fn kill_port(port: u16) -> ApiResponse<KillPortResult> {
    if port == 0 {
        return ApiResponse::failure(
            "Stub validation failed: port must be between 1 and 65535.",
            KillPortResult {
                attempted: false,
                port,
                note: "No process actions were performed.".to_string(),
            },
        );
    }

    ApiResponse::success(
        "Placeholder implementation: process termination is pending.",
        KillPortResult {
            attempted: false,
            port,
            note: "Command wiring works; kill logic will be added in Sprint 1.".to_string(),
        },
    )
}

