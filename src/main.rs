// ...existing code...
use std::io::{self, Write};

mod storage;
mod task; // All functions at
mod utils; // Print all stuff
mod test; // for tes

fn main() {
    // ensure DB exists
    storage::establish_connection().expect("Failed to initialize DB");

    loop {
        let mut choice = String::new();
        utils::menu(&mut choice);

        let choice_int: i32 = match choice.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("PLEASE ENTER A NUMBER");
                continue;
            }
        };

        // 0 = exit
        if choice_int == 0 {
            println!("Goodbye.");
            break;
        }

        task::handle_choice(choice_int);
        println!(); // spacing between iterations
    }
}