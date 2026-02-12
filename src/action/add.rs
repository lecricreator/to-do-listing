use std::{io::{/*self, BufRead, BufReader, */Write}, fs::File};
use crate::gestionary_file:: replace_file;
use crate::errors;
//use colored::Colorize;

pub fn add(argc: usize, args: &Vec<String>){
    if !errors::verified_arg(argc, 3) {return};
    replace_file(args, add_in_file, "add".to_string());
}

fn add_in_file (table_line: &Vec<String>, mut file_at_replace: &File, _input_index:usize, t: &usize){
    let line = format!("{}", table_line[*t]);
    file_at_replace.write(line.as_bytes()).expect("Can not write in file");
}

fn _remove_line(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize) {
        if *t != input_index + 3 {
            file_at_replace.write(table_line[*t].as_bytes()).expect("Can not write in file");
        }
}