use crate::errors;
use crate::gestionary_file;
use std::fs::File;
use std::io::Write;

pub fn remove(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {
        return;
    }
    gestionary_file::replace_file(args, remove_line, "remove")
}

fn remove_line(
    table_line: &Vec<String>,
    mut file_at_replace: &File,
    input_index: usize,
    t: &usize,
) {
    if *t != input_index + 3 {
        file_at_replace
            .write(table_line[*t].as_bytes())
            .expect("Can not write in file");
    }
}
