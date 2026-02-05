use std::{i32, io::{self, BufRead, BufReader, Write}};
use std::fs::{File};
use crate::gestionary_file::{self};

pub fn remove(argc: usize, args: &Vec<String>) -> io::Result<()>{
    if argc != 3{
        println!("Need 3 arguments.\n1: action \n2: task\n3. (optinal) commentary");
        return Ok(())
    }
    let mut file = match gestionary_file::find_file(args){
        Ok(f) => f,
        Err(e) => {
            println!("to-do-rustfile not exist.\nTap 'list' for see all the to-do-rustfile.{}", e);
            return Ok(())
        }
    };
    let reader = BufReader::new(&file);
    let mut index = 0;
    let mut table_line: Vec<String> = vec![];
    for line in reader.lines() {
        let mut line= line?;
        if index > 2 {
            print!("{} : ", index - 3)
        }else {
            print!("    ");
        }
        println!("{}", line);
        line += "\n";
        table_line.push(line);
        index += 1;
    }
    index -= 4;
    println!("Choose the index to remove. Ex 1");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input");
    let transf_input_to_int: usize = match input.trim().parse::<usize>() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Invalid number: {}", e);
            return Ok(());
        }
    };
    if transf_input_to_int > index {println!("value out of index of the to-do-rustlist."); return Ok(());}
    let mut file_at_replace:File = File::options()
    .write(true)
    .create(true)
    .open("replace_file")
    .expect("Cannot create the replace_file.");
    for t in 0..table_line.len(){
        if t != transf_input_to_int + 3 {
            file_at_replace.write(table_line[t].as_bytes()).expect("Can not write in file");
        }
    }
    return Ok(())
}