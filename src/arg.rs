use crate::action::add;
use crate::action::complete;
use crate::action::delete;
use crate::action::help;
use crate::action::list;
use crate::action::new;
use crate::action::remove;
use crate::action::show;
use crate::action::uncomplete;
use crate::errors;


pub fn start_program(args: &[String]) -> Result<(), errors::MyError> {
    let (action, args) = args.split_first().ok_or_else(|| errors::MyError::ActionNeeded)?;
    match action.as_str() {
        "new" => new::new_action(args)?,
        "show" => show::show(args)?,
        "add" => add::add(args)?,
        "help" => help::help(),
        "remove" => remove::remove(args)?,
        "complete" => complete::complete(args)?,
        "uncomplete" => uncomplete::uncomplete(args)?,
        "delete" => delete::delete(args)?,
        "list" => list::list()?,
        _ => Err(errors::MyError::ActionNotExist)?,
    }
    Ok(())
}
