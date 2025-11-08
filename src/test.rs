pub fn print_added_task() {
    // call the add function (it will prompt for input)
    let t = crate::task::add();
    println!("Added task: {:#?}",t );
}