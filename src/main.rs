mod task_manager;
use std::io;
use task_manager::TaskManager;

fn read_task_id() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(id) => Some(id),
        Err(_) => None,
    }
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

        let mut input = String::new();

        // TODO: Read the user's choice.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => {
                // TODO: Read title and call manager.add()
                println!("Enter task title.");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read input");

                manager.add(title.trim().to_string());
                println!("Task added successfully!");
            }
            "2" => manager.list(),
            "3" => {
                // TODO: Read task ID and call manager.complete()
                println!("Enter task ID to complete");

                match read_task_id() {
                    Some(id) => {
                        if manager.complete(id) {
                            println!("Task completed successfully!");
                        } else {
                            println!("Task Not Found");
                        }
                    }
                    None => println!("Please enter a valid task ID."),
                }
            }
            "4" => {
                // TODO: Read task ID and call manager.remove()
                println!("Enter task ID to remove");

                match read_task_id() {
                    Some(id) => {
                        if manager.remove(id) {
                            println!("Task removed successfully!");
                        } else {
                            println!("Task Not Found");
                        }
                    }
                    None => println!("Please enter a valid task ID."),
                }
            }
            "5" => break,
            _ => println!("Invalid option"),
        }
    }
}
