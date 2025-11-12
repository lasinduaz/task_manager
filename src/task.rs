// ...existing code...
use chrono::Local;
use std::io::{self, Write};

use crate::storage;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: i32,
    pub due_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub fn handle_choice(choice_int: i32) {
    match choice_int {
        1 => {
            let t = add();
            println!("Added: {:#?}", t);
        }
        2 => {
            if let Err(e) = view() {
                eprintln!("View error: {}", e);
            }
        }
        3 => {
            if let Err(_) = update() {
                // errors are printed inside update
            }
        }
        4 => {
            if let Err(_) = delete() {
                // errors printed inside delete
            }
        }
        _ => println!("Invalid choice: {}", choice_int),
    }
}

// collect input and insert into DB; returns Task (id set from DB)
pub fn add() -> Task {
    let mut title = String::new();
    let mut description = String::new();
    let mut status = String::new();
    let mut priority = String::new();
    let mut due_date = String::new();

    println!("================================");
    println!("Add tasks here");
    println!("================================");

    print!("Task Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).expect("read failed");

    print!("Task Description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).expect("read failed");

    print!("Task Status (pending/in_progress/done): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut status).expect("read failed");

    print!("Task Priority (1-3): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut priority).expect("read failed");

    print!("Task Due Date (optional): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut due_date).expect("read failed");

    let final_title = title.trim().to_owned();
    let final_description = {
        let d = description.trim();
        if d.is_empty() { None } else { Some(d.to_string()) }
    };
    let final_status = {
        let s = status.trim();
        if s.is_empty() { "pending".to_string() } else { s.to_string() }
    };
    let final_priority = priority.trim().parse::<i32>().unwrap_or(1);
    let final_due_date = {
        let d = due_date.trim();
        if d.is_empty() { None } else { Some(d) }
    };

    let id = storage::add_task(
        &final_title,
        final_description.as_deref(),
        &final_status,
        final_priority,
        final_due_date,
    )
    .expect("Failed to add task to database");

    Task {
        id: Some(id as i32),
        title: final_title,
        description: final_description,
        status: final_status,
        priority: final_priority,
        due_date: final_due_date.map(|s| s.to_string()),
        created_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        updated_at: None,
    }
}

pub fn view() -> Result<(), rusqlite::Error> {
    storage::view_tasks()
}

pub fn update() -> Result<(), ()> {
    let mut id_upt = String::new();
    println!();
    println!("================================");
    println!("Update tasks (status only)");
    println!("================================");
    print!("Please Enter ID number : ('#' for view tasks) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_upt).expect("Failed to get ID");

    let input = id_upt.trim();
    if input == "#" {
        storage::view_tasks().map_err(|e| { eprintln!("{}", e); () })?;
        return Ok(());
    }

    let id: i32 = match input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("PLEASE ENTER A NUMBER");
            return Err(());
        }
    };

    match storage::get_task_status(id) {
        Ok(None) => {
            println!("There is no task with ID {}", id);
            return Err(());
        }
        Ok(Some(current_status)) => {
            println!("Current Status: {}", current_status);
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            return Err(());
        }
    }

    let mut new_status = String::new();
    print!("Enter new status ('pending', 'in_progress', 'done'): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_status).expect("Failed to read new status");
    let new_status = new_status.trim();
    if new_status.is_empty() {
        println!("No status entered. Aborting update.");
        return Err(());
    }

    match storage::update_task_status(id, new_status) {
        Ok(0) => println!("No rows updated (id {}).", id),
        Ok(_) => println!("Task {} status updated to '{}'.", id, new_status),
        Err(e) => {
            eprintln!("Failed to update status: {}", e);
            return Err(());
        }
    }
    Ok(())
}

pub fn delete() -> Result<(), ()> {
    let mut id_dete = String::new();
    println!();
    println!("================================");
    println!("Delete tasks here");
    println!("================================");
    print!("Please Enter ID number : ('#' for view tasks) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_dete).expect("unable to capture input");

    if id_dete.trim() == "#" {
        storage::view_tasks().map_err(|e| { eprintln!("{}", e); () })?;
        return Ok(());
    }

    let id_mut: i32 = match id_dete.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid integer id");
            return Err(());
        }
    };

    match storage::delete_task(id_mut) {
        Ok(0) => println!("No task with id {}", id_mut),
        Ok(_) => println!("Task deleted successfully!"),
        Err(e) => eprintln!("Failed to delete task: {}", e),
    }

    Ok(())
}