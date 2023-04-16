#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs::{self, File},
    path::Path,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_data, save_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("{}", name);
    format!("Hello, {}!", name)
}

// invoke 接收的参数名称不能带下划线
#[tauri::command]
fn load_data(exefilepath: &str) -> String {
    println!("host = {}", exefilepath);
    // if !Path::new("./appdata.dat").exists() {
    //     return "当前目录下未找到appdata.dat 文件".to_string();
    // } else {
    //     return exe_file_path.to_string();
    // }

    return exefilepath.to_string();
    // let file_path = "";

    // let f_str = fs::read_to_string(file_path).unwrap();

    // format!("Hello, {}!", 1)
}

#[tauri::command]
fn save_data() -> String {
    format!("Hello, {}!", 1)
}
