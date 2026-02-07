pub fn help() {
    println!("To-do-rustlist is a to-do-list in the terminal.\n");
    println!("new        : Create a new to-do-rustlist.
            todorustlist new name_of_todorustlist_file.\n");
    println!("add        : Add a new task to complete and (optionnal) add a commentary for mor details.
            todorustlist add name_of_todorustlist_file write_task (optionnal)write_comentary\n");
    println!("show       : Show the todorustlist for see the progression.
            todorustlist show name_of_todorustlist_file\n");
    println!("remove     : remove a task, after you can choose the task at remove.
            todorustlist remove name_of_todorustlist_file\n");
    println!("complete   : complete a task.
            todorustlist complete name_of_todorustlist_file\n");
    println!("uncomplete : uncomplete a task. After you can choose the task at remove.
            todorustlist uncomplete name_of_todorustlist_file\n");
    println!("file        : file permit you to see all the todorustlist file you create.
            todorustlist file\n");
    println!("delete      : delete permit you to delete a todorustlist.
            todorustlist delete name_of_todorustlist_file\n");
}