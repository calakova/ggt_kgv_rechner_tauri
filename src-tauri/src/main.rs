// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::ParseIntError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calc(num1: &str, num2: &str) -> String {
    let a = parse_input_to_int(read_number_input(num1)).abs();
    let b = parse_input_to_int(read_number_input(num2)).abs();

    let ggt = euclid(a, b);
    let kgv = (a * b) / ggt;

    format!("ggT: {}, kgV: {}", ggt, kgv)
}

fn read_number_input(input: &str) -> Result<i32, ParseIntError> {
    let number: i32 = input.trim().parse()?;
    Ok(number)
}

fn parse_input_to_int(result: Result<i32, ParseIntError>) -> i32 {
    match result {
        Ok(num) => num,
        Err(_) => -1
    }
}

fn euclid(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let h = a % b;
        a = b;
        b = h;
    }
    a
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

