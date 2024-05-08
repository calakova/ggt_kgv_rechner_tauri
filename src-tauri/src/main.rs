// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::ParseIntError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calc(num1: &str, num2: &str) -> String {
    let a = parse_input_to_int(read_number_input(num1));
    let b = parse_input_to_int(read_number_input(num2));

    let ggt = euclid(a, b);
    let kgv = kgv_calc(a, b, ggt);

    format!("ggT: {}, kgV: {}", ggt, kgv)
}

fn read_number_input(input: &str) -> Result<u64, ParseIntError> {
    let number: u64 = input.trim().parse()?;
    Ok(number)
}

fn parse_input_to_int(result: Result<u64, ParseIntError>) -> u64 {
    match result {
        Ok(num) => num,
        Err(_) => 0
    }
}

fn euclid(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let h = a % b;
        a = b;
        b = h;
    }
    a
}

fn kgv_calc(a: u64, b: u64, ggt: u64) -> String {
    let error_str = "Ergebnis zu groß".to_string();

    let product = match a.checked_mul(b) {
        Some(n) => n,
        None => {
            0
        }
    };

    if product == 0 {
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

