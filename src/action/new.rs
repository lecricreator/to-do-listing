use crate::gestionary_file;

pub fn new_action(args: &[String]) {
    let Some(file_name) = args.first() else {
        return;
    };

    gestionary_file::create_file(file_name);
}