use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_profile,
            get_profile,
            get_remote_profile,
            run_sing_box,
            stop_sing_box
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_working_dir() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not get home directory");
    let config_path: PathBuf = home_dir.join(".config/sing-box/");

    config_path
}

fn get_profile_path() -> PathBuf {
    let home_dir = get_working_dir();
    let config_path: PathBuf = home_dir.join("config.json");
    config_path
}

#[tauri::command]
fn get_profile() -> String {
    let config_path: PathBuf = get_profile_path();
    let config_json = fs::read_to_string(config_path).unwrap();

    config_json
}

#[tauri::command]
fn set_profile(profile: String) {
    let config_path: PathBuf = get_profile_path();

    fs::write(config_path, profile).unwrap();
}

#[tauri::command]
fn get_remote_profile(url: String) -> String {
    let profile = reqwest::blocking::get(url)
        .expect("Could not get profile")
        .text()
        .unwrap();
    let config_path: PathBuf = get_profile_path();

    fs::write(config_path, profile.clone()).unwrap();

    profile.clone()
}

#[tauri::command]
fn run_sing_box() {
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg("sing-box")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
}


#[tauri::command]
fn stop_sing_box() {
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("sing-box")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
}
