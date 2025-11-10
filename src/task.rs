use chrono::Local;
use core::task;
use std::io::{self, Write};

use crate::{storage::{self, view_tasks}, utils};

#[derive(Debug)]

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

    print!("Task Name :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read title");

    print!("Task Desciption :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description");

    print!("Task Status('pending', 'in_progress', 'done') :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut status)
        .expect("Failed to read status");

    print!("Task Priority ('1' - Higt ,'2' - Medium,  '3' - Low , ) :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut priority)
        .expect("Failed to read priority");
    let priority_val = priority.trim().parse::<i32>().unwrap_or(1); //get int answer 

    print!("Task Due Date (YYYY-MM-DD) :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut due_date)
        .expect("Failed to read due date");

    //get syst date
    let now = Local::now();
    created_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
    updated_at = now.format("%Y-%m-%d %H:%M:%S").to_string();

    //show tasks
    println!("");
    println!("================================");
    println!("Task Name : {}", title.trim());
    println!("Task Description :{}", description.trim());
    println!("Task Status : {}", status.trim());
    println!("Task Priority : {}", priority_val);
    println!("Task Due Date : {}", due_date.trim());
    println!("Task Created At : {}", created_at.trim());
    println!("Task Updated At : {}", updated_at.trim());
    println!("================================");
    println!("");

    // normalize inputs into concrete types before constructing Task
    let final_title = title.trim().to_string();
    let final_description = description.trim().to_string();
    let final_status = status.trim().to_string();
    let final_priority = priority.trim().parse::<i32>().unwrap_or(3);
    let final_due_date = due_date.trim().to_string();

    //insert values to D&B
    let db_id = storage::add_task(
        &final_title,
        &final_description,
        &final_status,
        final_priority,
        if final_due_date.is_empty() {
            None
        } else {
            Some(final_due_date.as_str())
        },
    )
    .expect("Failed to add task to database");

    Task {
        ID: db_id as i32,
        title: final_title,
        description: final_description,
        status: final_status,
        priority: final_priority,
        due_date: final_due_date,
        created_at,
        updated_at,
    }
}

pub fn view() {
    storage::view_tasks().expect("Failed to view tasks");
}
pub fn update() {
    let mut id_upt = String::new();
    println!();
    println!("================================");
    println!("Update tasks here");
    println!("================================");
    println!("Please Enter ID number : ('#' for view tasks)");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_upt).expect("Failed to get ID");

    let input = id_upt.trim();
    if input == "#" {
        storage::view_tasks().expect("Failed to get view task");
        return;
    }

    // convert to int
    let id: i32 = match input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("PLEASE ENTER A NUMBER");
            return;
        }
    };

    // fetch existing status
    match crate::storage::get_task_status(id) {
        Ok(None) => {
            println!("There is no task with ID {}", id);
            return;
        }
        Ok(Some(current_status)) => {
            println!("=================================");
            println!("Current Status: {}", current_status);
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            return;
        }
    }

    // prompt for new status
    let mut new_status = String::new();
    print!("Enter new status ('pending', 'in_progress', 'done'): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut new_status)
        .expect("Failed to read new status");
    let new_status = new_status.trim();
    if new_status.is_empty() {
        println!("No status entered. Aborting update.");
        return;
    }

    // update status in DB
    match crate::storage::update_task_status(id, new_status) {
        Ok(0) => println!("No rows updated (id {}).", id),
        Ok(_) => println!("Task {} status updated to '{}'.", id, new_status),
        Err(e) => eprintln!("Failed to update status: {}", e),
    }

    //return to menu

    let mut choice = String::new();
    utils::menu(&mut choice);
}

pub fn delete() {
    println!("Delete tasks here");
}

//for update funtion
// - First need to get id number
// - Secound sent ID number to update functio at stroage for get data
// - print those on on here funtion
