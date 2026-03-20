use crate::core::response::ApiResponse;
use crate::modules::system::hosts;
use crate::modules::system::hosts::{AddHostRequest, HostEntry, HostMutationResult};
use crate::modules::system::ports;
use crate::modules::system::ports::{KillPortResult, PortInfo};

#[tauri::command]
pub async fn list_hosts() -> ApiResponse<Vec<HostEntry>> {
    hosts::list_hosts()
}

#[tauri::command]
pub async fn add_host(request: AddHostRequest) -> ApiResponse<HostMutationResult> {
    hosts::add_host(request)
}

#[tauri::command]
pub async fn remove_host(hostname: String) -> ApiResponse<HostMutationResult> {
    hosts::remove_host(hostname)
}

#[tauri::command]
pub async fn clean_hosts() -> ApiResponse<HostMutationResult> {
    hosts::clean_hosts()
}

#[tauri::command]
pub async fn list_ports() -> ApiResponse<Vec<PortInfo>> {
    ports::list_ports()
}

#[tauri::command]
pub async fn kill_port(port: u16) -> ApiResponse<KillPortResult> {
    ports::kill_port(port)
}

