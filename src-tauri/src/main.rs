// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod pinyin_converter;

use pinyin_converter::word_to_pinyin;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![word_to_pinyin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
