// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod models; 
pub mod services; 
pub mod core;

#[tokio::main] 
async fn main() {
    _ = db::connection::init_db().await;
    genplanner_lib::run()
}



