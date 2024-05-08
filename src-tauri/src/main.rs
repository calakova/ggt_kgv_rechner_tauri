// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::ParseIntError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calc(num1: &str, num2: &str) -> String {
    let a = parse_input_to_int(read_number_input(num1)).abs();
    let b = parse_input_to_int(read_number_input(num2)).abs();

    let ggt = euclid(a, b);
    let kgv = kgv_calc(a, b, ggt);

    format!("ggT: {}, kgV: {}", ggt, kgv)
}

fn read_number_input(input: &str) -> Result<i64, ParseIntError> {
    let number: i64 = input.trim().parse()?;
    Ok(number)
}

fn parse_input_to_int(result: Result<i64, ParseIntError>) -> i64 {
    match result {
        Ok(num) => num,
        Err(_) => -1
    }
}

fn euclid(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let h = a % b;
        a = b;
        b = h;
    }
    a
}

fn kgv_calc(a: i64, b: i64, ggt: i64) -> String {
    let error_str = "Ergebnis zu groß".to_string();
    
    let product = match a.checked_mul(b) {
        Some(n) => n,
        None => {
            -1
        }
    };

    if product == -1 {
        return error_str
    }

    let kgv = match product.checked_div(ggt) {
        Some(n) => n.to_string(),
        None => {
            error_str
        }
    };
    
    kgv
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

