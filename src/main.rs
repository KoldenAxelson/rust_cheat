mod cheatsheet;
mod highlighting;
mod manager;

use manager::CheatSheetManager;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let manager = CheatSheetManager::new();

    let result = match args.len() {
        1 => {
            manager.show_available_sheets();
            Ok(())
        }
        2 => match args[1].parse::<usize>() {
            Ok(index) => manager.show_sheet_outline(index),
            Err(_) => Err("Sheet index must be a positive integer".into()),
        },
        3 => match args[1].parse::<usize>() {
            Ok(sheet_index) => {
                if args[2] == "0" {
                    manager.show_full_sheet(sheet_index)
                } else {
                    manager.show_section(sheet_index, &args[2])
                }
            }
            Err(_) => Err("Sheet index must be a positive integer".into()),
        },
        _ => Err("Usage: rust_cheat [sheet_index] [section_number]".into()),
    };

    if let Err(e) = result {
        eprintln!("{}", manager.format_error(&e.to_string()));
        process::exit(1);
    }
}
