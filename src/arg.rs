use crate::action::add;
use crate::action::complete;
use crate::action::delete;
use crate::action::help;
use crate::action::list;
use crate::action::new;
use crate::action::remove;
use crate::action::show;
use crate::action::uncomplete;

pub fn start_program(args: &[String]) {
    let Some((action, args)) = args.split_first() else {
        return;
    };

    match action.as_str() {
        "new" => new::new_action(args),
        "show" => show::show(args),
        "add" => let _ = add::add(args),
        "help" => help::help(),
        "remove" => remove::remove(args),
        "complete" => complete::complete(args),
        "uncomplete" => uncomplete::uncomplete(args),
        "delete" => delete::delete(args),
        "list" => list::list(),
    }

    if action == "new" {
        new::new_action(argc, args);
    } else if action == "show" {
        show::show(argc, args);
    } else if action == "add" {
        let _ = add::add(argc, args);
    } else if action == "help" {
        help::help();
    } else if action == "remove" {
        remove::remove(argc, args);
    } else if action == "complete" {
        complete::complete(argc, args);
    } else if action == "uncomplete" {
        uncomplete::uncomplete(argc, args);
    } else if action == "delete" {
        delete::delete(argc, args);
    } else if action == "list" {
        list::list();
    } else {
        println!("This command doesn't exist in the to-do-rustline.");
    }
}
