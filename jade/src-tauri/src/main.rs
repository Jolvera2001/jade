// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod PieceTable;

use engine::*;

fn main() {
    jade_lib::run()
}
