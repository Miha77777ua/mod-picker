use std::fs;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn toggle(mut path: String, typeOfFile: String) {
    let startingPath = path.clone();

    if typeOfFile == "enable" {
        fs::rename(startingPath, path + ".dis").expect("Can'r toggle :(");    
    } else { 
        let len = path.chars().count();
        path.drain(len - 4..len);
        fs::rename(startingPath, path).expect("Can't toggle :(");
    } 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![toggle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
