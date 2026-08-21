// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod models; 
pub mod db; 

use core::dag::{Dag,Snapshot};
use db::connection::{establish_connection};
use models::goal::{get_goal};
use serde_json::{Value};
#[tauri::command] 
async fn get_snapshot(goal_id:String) -> anyhow::Result<Value>{
    let mut dag = Dag::new(&goal_id).await?;
    dag.download_dag().await;
    Ok(dag.to_snapshot())
}


#[tauri::command]
fn greet(name: &str) -> String {
    let return_str = format!("Hello, {}! You've been greeted from Rust!", name);
    return_str
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
