// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod models; 
pub mod db; 

use core::dag::{Dag,Snapshot};
use db::connection::{establish_connection};
use models::goal::{get_goal};
use serde_json::{Value};
use core::operation::{Op,apply_op};





#[tauri::command]
async fn get_snapshot(goal_id: String) -> Result<Value, String> {
    let mut dag = Dag::new(&goal_id).await.map_err(|err| format!("{err:?}"))?;

    dag.download_dag().await.map_err(|err| {
        eprintln!("get_snapshot error: {err:?}");
        format!("{err:?}")
    })?;

    serde_json::to_value(dag.to_snapshot()).map_err(|err| format!("{err:?}"))
}


//Todo make proper returning interface for apply opp
//Figure out how to keep a map of dags persistent in memory and load it
#[tauri::command]
async fn execute_op(op:Op)->Result<()>{
    
    //apply_op(dag_map, pool, &op);
    Ok(())
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
        .invoke_handler(tauri::generate_handler![greet,get_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
