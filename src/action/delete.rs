use std::{fs, io::{Write}};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn delete(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {return}
    match gestionary_file::find_file(&args[2]) {
        Ok(l) => l,
        Err(_e) => return,
    };
    fs::remove_file("a.txt").expect("Cannot remove file.");
}