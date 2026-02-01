use crate::action::new;
use crate::action::show;
use crate::action::add;

pub fn start_program(argc: usize, args: &Vec<String>){
    let action = &args[1];
    if action == "new" {
        new::new_action(argc, args);
    }else if action == "show" {
        show::show(argc, args);
    }else if action == "add" {
        let _ = add::add(argc, args);
    }else {
        println!("This command doesn't exist in the to-do-rustline.");
    }

}