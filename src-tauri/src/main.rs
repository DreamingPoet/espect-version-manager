#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod crypt_lib;
use std::{
    fs::{self, File, OpenOptions},
    path::Path, io::Write,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VersionData {
    custom_name: String,
    custom_logo: String,
    app_name: String,
    app_logo: String,
    version: String,
    internal_version: String,
    develop_company_name: String,
    develop_company_name_en: String,
    contact_email: String,
    copy_right: String,
    commit_sha: String,
}

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
    let path = Path::new(exefilepath);
    
    if let Some(ue_project_dir) = path.parent(){
        if let Some(data_file_path) = ue_project_dir.to_str(){
            let data_file_path = data_file_path.to_string() + "appdata.dat";
            
            if !Path::new(&data_file_path).exists() {

                let _ = File::create(&data_file_path);
                return "当前目录下未找到appdata.dat 文件, 已新建！".to_string();
            } else {
                return exefilepath.to_string();
            }
        }

    }else {
        return format!("save file failed, path error {}!", &exefilepath);
    }




    return exefilepath.to_string();
    // let file_path = "";

    // let f_str = fs::read_to_string(file_path).unwrap();

    // format!("Hello, {}!", 1)
}

#[tauri::command]
fn save_data(
    exefilepath: &str,
    customname: &str,
    customlogo: &str,
    appname: &str,
    applogo: &str,
    version: &str,
    internalversion: &str,
    developcompanyname: &str,
    developcompanynameen: &str,
    contactemail: &str,
    copyright: &str,
    commitsha: &str,
) -> String {

    let test_data = VersionData {
        custom_name: customname.to_string(),
        custom_logo: customlogo.to_string(),
        app_name: appname.to_string(),
        app_logo: applogo.to_string(),
        version: version.to_string(),
        internal_version: internalversion.to_string(),
        develop_company_name: developcompanyname.to_string(),
        develop_company_name_en: developcompanynameen.to_string(),
        contact_email: contactemail.to_string(),
        copy_right: copyright.to_string(),
        commit_sha: commitsha.to_string(),
    };
    
    let mut data_str = serde_json::to_string_pretty(&test_data).unwrap();

    data_str = crypt_lib::encrypt(&data_str);

    let path = Path::new(exefilepath);
    
    if let Some(ue_project_dir) = path.parent(){
        if let Some(data_file_path) = ue_project_dir.to_str(){
            let data_file_path = data_file_path.to_string() + "appdata.dat";
            
            save_data_to_file(&data_str, &data_file_path);
            return format!("parent is , {:?}!", &ue_project_dir);
        }

    }else {
        return format!("save file failed, path error {}!", &exefilepath);
    }


    return format!("save file failed, path error {}!", &exefilepath);
}


fn save_data_to_file(file_str: &str, path: &str) {
    println!("save file {}", &path);
    let mut options = OpenOptions::new();
    if let Ok(mut file) = options.write(true).open(path){
        let _ = file.write(file_str.as_bytes());
    } else {
        println!("save file failed!! {}", &path);
    }
}


#[test]
fn aes_test() {
    let test_data = VersionData {
        custom_name: "VR互动教学系统".to_string(),
        custom_logo: "images/base64:dsfaew".to_string(),
        app_name: "".to_string(),
        app_logo: "".to_string(),
        version: "".to_string(),
        internal_version: "".to_string(),
        develop_company_name: "北京纳虚光影科技有限公司".to_string(),
        develop_company_name_en: "".to_string(),
        contact_email: "".to_string(),
        copy_right: "".to_string(),
        commit_sha: "".to_string(),
    };
    let data_str = serde_json::to_string_pretty(&test_data).unwrap();

    let a = crypt_lib::encrypt(&data_str);

    println!("{}", &a);
    
    let dst = crypt_lib::decrypt(&a);
    
    println!("{}", &dst);

}
