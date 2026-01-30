/*use core::error;
use std::fs::File;
use std::io::{self, Read};

fn main(){
    let file = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open("trya.todoR");
    println!("{:?}", file);
    match read_file("trya.todoR".to_string()){
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file(str_path: String) -> Result<String, io::Error>{
    let mut fd = File::open(str_path)?;
    let mut content: String = String::new();
    fd.read_to_string(&mut content)?;
    Ok(content)
    
}*/


/* WANT THIS RESULT, BUT NOT WORKING !!!*/
use std::fs::File;
use std::io::{self, Read};

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let action = &args[1];
    println!("{action}");
    let file = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open("trya.todoR");
    println!("{file:?}");
    let return_value = read_file("trya.todoR".to_string());
    let file_content: String = match return_value {
        Err(e) => {
            println!("Error: {e}");
            return;
        },
        Ok(s) => s,
    };
    println!("{file_content}")
}

fn read_file(str_path: String) -> Result<String, io::Error>{
    let mut fd = File::open(str_path)?;
    let mut content: String = String::new();
    fd.read_to_string(&mut content)?;
    Ok(content)
}
 