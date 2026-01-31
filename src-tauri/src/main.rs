// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

fn main() {
    if let Err(e) = db::init_db() {
        panic!("Database init failed: {:?}", e);
    }

    println!("SQLite ready");

    yuno_tasks_lib::run();
}
