use serde::{Deserialize, Serialize};

use crate::core::response::ApiResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEntry {
    pub ip: String,
    pub hostname: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddHostRequest {
    pub ip: String,
    pub hostname: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostMutationResult {
    pub applied: bool,
    pub target: Option<String>,
    pub note: String,
}

pub fn list_hosts() -> ApiResponse<Vec<HostEntry>> {
    ApiResponse::success(
        "Placeholder implementation: hosts file parsing will be added in Sprint 1.",
        Vec::new(),
    )
}

pub fn add_host(request: AddHostRequest) -> ApiResponse<HostMutationResult> {
    let hostname = request.hostname.trim().to_owned();
    let ip = request.ip.trim().to_owned();

    if hostname.is_empty() || ip.is_empty() {
        return ApiResponse::failure(
            "Stub validation failed: both ip and hostname are required.",
            HostMutationResult {
                applied: false,
                target: None,
                note: "No file changes were made.".to_string(),
            },
        );
    }

    ApiResponse::success(
        "Placeholder implementation: no hosts file changes yet.",
        HostMutationResult {
            applied: false,
            target: Some(format!("{ip} {hostname}")),
            note: "Command wiring works; persistence is pending.".to_string(),
        },
    )
}

pub fn remove_host(hostname: String) -> ApiResponse<HostMutationResult> {
    let target = hostname.trim().to_owned();
    if target.is_empty() {
        return ApiResponse::failure(
            "Stub validation failed: hostname is required.",
            HostMutationResult {
                applied: false,
                target: None,
                note: "No file changes were made.".to_string(),
            },
        );
    }

    ApiResponse::success(
        "Placeholder implementation: no hosts file changes yet.",
        HostMutationResult {
            applied: false,
            target: Some(target),
            note: "Removal logic will be implemented in Sprint 1.".to_string(),
        },
    )
}

pub fn clean_hosts() -> ApiResponse<HostMutationResult> {
    ApiResponse::success(
        "Placeholder implementation: deduplication and cleanup not implemented yet.",
        HostMutationResult {
            applied: false,
            target: None,
            note: "No file changes were made.".to_string(),
        },
    )
}

