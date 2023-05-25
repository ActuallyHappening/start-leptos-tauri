#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::{command, generate_context, generate_handler};
use log::info;

#[command]
async fn run_ble_example() {
	info!("[tauri] run_ble_example command executing ...");
	
	println!("You should see this on your native console!");
}

fn main() {
	pretty_env_logger::init();
	info!("Running tauri application ...");

	tauri::Builder::default()
		.invoke_handler(generate_handler![run_command_example])
		.run(generate_context!())
		.expect("error while running tauri application");
}
