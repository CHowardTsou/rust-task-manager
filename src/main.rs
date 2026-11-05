mod task_manager;
use std::io;
use task_manager::TaskManager;

/// Reads one line from stdin.
///
/// Returns `None` when stdin has reached end of input or a read fails
/// (for example, on input that is not valid UTF-8), so callers can stop
/// instead of looping on empty input or panicking.
fn read_line() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => Some(input),
        Err(err) => {
            eprintln!("Failed to read input: {err}");
            None
        }
    }
}

/// Reads a task ID. The outer `Option` is `None` when input has ended;
/// the inner one is `None` when the line is not a valid ID.
fn read_task_id() -> Option<Option<u32>> {
    read_line().map(|input| input.trim().parse::<u32>().ok())
}

fn main() {
    let mut manager = TaskManager::new();

    loop {
        println!("\n===== TASK MANAGER =====");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Remove task");
        println!("5. Exit");

        let Some(input) = read_line() else { break };

        match input.trim() {
            "1" => {
                println!("Enter task title.");
                let Some(title) = read_line() else { break };

                manager.add(title.trim().to_string());
                println!("Task added successfully!");
            }
            "2" => manager.list(),
            "3" => {
                println!("Enter task ID to complete");

                match read_task_id() {
                    None => break,
                    Some(Some(id)) => {
                        if manager.complete(id) {
                            println!("Task completed successfully!");
                        } else {
                            println!("Task Not Found");
                        }
                    }
                    Some(None) => println!("Please enter a valid task ID."),
                }
            }
            "4" => {
                println!("Enter task ID to remove");

                match read_task_id() {
                    None => break,
                    Some(Some(id)) => {
                        if manager.remove(id) {
                            println!("Task removed successfully!");
                        } else {
                            println!("Task Not Found");
                        }
                    }
                    Some(None) => println!("Please enter a valid task ID."),
                }
            }
            "5" => break,
            _ => println!("Invalid option"),
        }
    }
}
