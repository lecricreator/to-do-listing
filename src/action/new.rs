use crate::gestionary_file;
use crate::errors::{self};

pub fn new_action(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {return;}
    gestionary_file::create_file(&args[2]);
}