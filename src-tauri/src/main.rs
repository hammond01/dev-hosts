#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod modules;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::system::list_hosts,
            commands::system::add_host,
            commands::system::remove_host,
            commands::system::clean_hosts,
            commands::system::list_ports,
            commands::system::kill_port
        ])
        .run(tauri::generate_context!())
        .expect("failed to run DevHosts");
}

