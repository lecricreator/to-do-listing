use std::{io::{Write}};
use std::fs::{File};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn complete(argc: usize, args: &Vec<String>){
    if !errors::verified_arg(argc, 3) {return};
    gestionary_file::replace_file(args, complete_file, "complete task");
}

fn complete_file(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize){
    if *t == input_index + 3 {
        let modify_str = table_line[*t].replace("\u{274C}", "\u{2705}");
        file_at_replace.write(modify_str.as_bytes()).expect("Can not write the modification in file.");
    }else{
        file_at_replace.write(table_line[*t].as_bytes()).expect("Can not write in file.");
    }

}