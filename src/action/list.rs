use std::fs;
use std::io;

use crate::action::new;

pub fn list() -> io::Result<()>{
    let mut table_str: Vec<String> = Vec::new();
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(name) = file_name.to_str() {
            if name.ends_with(".todoR") {
                table_str.push(name.to_string());
            }
        }
    }
    println!("You have {} todoR.", table_str.len());
    for i in table_str {
        println!("{i}");
    }
    Ok(())
}