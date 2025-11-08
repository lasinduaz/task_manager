use std::io::{self, Write};

mod storage;
mod task; // All functions at
mod utils; // Print all stuff
mod test; // for tes

fn main() {
    // db connection — handle errors
    storage::establish_connection().expect("Failed to initialize DB");

    let mut choice = String::new();
    utils::menu(&mut choice);

    // parsing 
    let choice_int: i32 = match choice.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("PLEASE ENTER A NUMBER");
            return;
        }
    };

    // ensure this function name matches the one in src/task.rs
    task::handle_choies(choice_int);





    test::print_added_task();
}
