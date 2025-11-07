use chrono::Local;
use std::io::{self, Write};

pub struct Task {
    ID: i32,
    title: String,
    description: String,
    status: String,
    priority: i32,
    due_date: String,
    created_at: String,
    updated_at: String,
}

pub fn handle_choies(choice_int: i32) {
    match choice_int {
        0 => {
            println!("Stopped");
            std::process::exit(0);
        }
        1 => {
            let _ = add();
        }
        2 => view(),
        3 => update(),
        4 => delete(),
        _ => println!("Invalid choice: {}", choice_int),
    }
}
//operations

pub fn add() -> Task {
    let mut title = String::new();
    let mut description = String::new();
    let mut status = String::new();
    let mut priority = String::new();
    let mut due_date = String::new();
    let mut created_at = String::new();
    let mut updated_at = String::new();
    //let mut buf = String::new();

    println!("================================");
    println!("Add tasks here");
    io::stdout().flush().unwrap();
    println!("================================");

    println!("Task Name :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).expect("Failed to read title");

    print!("Task Desciption");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).expect("Failed to read description");

    print!("Task Status");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut status).expect("Failed to read status");

    println!("Tasl Priority ('1' - Higt , '3' - Low , '2' - Medium)");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut priority).expect("Failed to read priority");
    let priority_val = priority.trim().parse::<i32>().unwrap_or(1); //get int answer 

    println!("Task Due Date");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut due_date).expect("Failed to read due date");

    //get sys date
    let now = Local::now();
    created_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
    updated_at = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // build and return the Task
    Task {
        ID: 0,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        status: status.trim().to_string(),
        priority: priority.trim().parse::<i32>().unwrap_or(3),
        due_date: due_date.trim().to_string(),
        created_at,
        updated_at,
    }
}
pub fn view() {
    println!("View tasks here");
}
pub fn update() {
    println!("Update tasks here");
}
pub fn delete() {
    println!("Delete tasks here");
}