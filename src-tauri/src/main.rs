// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sqlx::{migrate::MigrateDatabase, Sqlite}; 
mod db;

fn main() {
    genplanner_lib::run()
}



