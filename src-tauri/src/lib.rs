// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod models; 
pub mod db; 
pub mod state;

use core::dag::{Dag,Snapshot};
use std::{collections::HashMap, ops::DerefMut};
use db::connection::{establish_connection};
use models::goal::{get_goal};
use serde_json::{Value};
use models::operation::{Op,apply_op,get_valid_dag};

use state::state::{AppState,initilize_state};

use tauri::{Builder, Manager,State};
use tokio::sync::Mutex;

use crate::models::goal;

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
async fn execute_op(state:State<'_,Mutex<AppState>>,op:Op,base_version:i64,goal_id: Option<String>)->Result<(Vec<Op>,i64,Value),String>{
    let mut mut_gaurd = state.lock().await;
    let state = mut_gaurd.deref_mut();    
    let pool = establish_connection().await.map_err(|err|format!("{err:?}") )?;
    let ops = apply_op(&mut state.dag_map, &pool, &op, &base_version, &goal_id).await.map_err(|err| format!("{err:?}"));
    ops

}

#[tauri::command]
fn greet(name: &str) -> String {
    let return_str = format!("Hello, {}! You've been greeted from Rust!", name);
    return_str
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            let handle = app.handle().clone();
    

            tauri::async_runtime::block_on(async move {
                let mut app_state = AppState::default();
                initilize_state(&mut app_state).await.unwrap();
                handle.manage(Mutex::new(app_state));

            });
 
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,get_snapshot,execute_op])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
