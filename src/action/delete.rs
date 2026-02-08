use std::{fs, io::{Write}};

fn delete() {
    fs::remove_file("a.txt")?;
}