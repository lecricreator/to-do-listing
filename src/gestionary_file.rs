use std::io::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn read_file(fd: &mut File) -> String{
    let mut content: String = String::new();
    fd.read_to_string(&mut content).expect("Error cannot read_to_string in function read_file | file : gestionary");
    content
}

pub fn create_file(name_file: &String){
    let total_name_file: String = format!("{name_file}.todoR");
    if Path::new(&total_name_file).exists() {
        println!("No need to create this files. The {total_name_file} is already exist.");
        return;
    }
    let mut file = File::options()
    .write(true)
    .create(true)
    .open(&total_name_file)
    .expect("Cannot create the {total_name_file}.");
    println!("Create the file {total_name_file}.\nNow you can add for add goal or show for showing the to-do-rustlist.");
    let _ = writeln!(file, "{}\nDONE |        TASK        | COMMENTARY        ", &name_file).expect("Cannot write in the file {total_name_file}.");
    _ = writeln!(file,       "------------------------------------------------").expect("Cannot write in the file {total_name_file}.");
}

pub fn find_file(args: &Vec<String>) -> Result<File, Error>{
    let total_file_name:String = format!("{}.todoR", &args[2]);
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&&args[2])
        .or_else(|_| {
            OpenOptions::new()
            .read(true)
            .write(true)
            .open(&total_file_name)
        })
}