use std::io::{self, Write};

pub fn menu(choice: &mut String) {
    choice.clear();
    println!("================================");
    println!("     To Do List");
    println!("================================");
    println!("0. Exit");
    println!("1. Add Task");
    println!("2. View Tasks");
    println!("3. Update Task");
    println!("4. Delete Task");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(choice).expect("failed to read line");
}