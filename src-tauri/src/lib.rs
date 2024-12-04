
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
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
    .invoke_handler(tauri::generate_handler![set_profile, get_profile, get_remote_profile])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_profile_path() -> std::path::PathBuf {
  let home_dir = dirs::home_dir().expect("无法获取用户主目录");
  let config_path: std::path::PathBuf = home_dir.join(".config/sing-box/config.json");
  config_path
}

#[tauri::command]
fn get_profile() -> String {
  use std::fs;
  use std::path::PathBuf;

  let config_path: PathBuf = get_profile_path();
  let config_json = fs::read_to_string(config_path).unwrap();

  println!("{}", config_json);
  config_json
}


#[tauri::command]
fn set_profile(profile: String) {
  use std::fs;
  use std::path::PathBuf;

  // 获取用户主目录
  let config_path: PathBuf = get_profile_path();

  fs::write(config_path, profile).unwrap();
}

#[tauri::command]
fn get_remote_profile(url: String) -> String {
  use std::fs;
  use std::path::PathBuf;

  let profile = reqwest::blocking::get(url).expect("无法获取配置文件").text().unwrap();
  let config_path: PathBuf = get_profile_path();

  fs::write(config_path, profile.clone()).unwrap();

  profile.clone()
}
