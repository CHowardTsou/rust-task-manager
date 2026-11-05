pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, title: String) {
        let next_id = self.next_id.checked_add(1).expect("Task ID overflow");

        self.tasks.push(Task {
            id: self.next_id,
            title,
            completed: false,
        });
        self.next_id = next_id;
    }
    pub fn complete(&mut self, id: u32) -> bool {
        // Find the task with this ID.
        // Change completed from false to true.
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.completed = true;
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, id: u32) -> bool {
        // TODO: Find the task by ID
        // TODO: Remove it from the vector
        // TODO: Return true if removed, false otherwise

        let position = self.tasks.iter().position(|x| x.id == id);
        if let Some(index) = position {
            self.tasks.remove(index);
            return true;
        }
        false
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("No Tasks Found.");
        } else {
            println!("My Task");
            for task in self.tasks.iter() {
                // Print task ID, title, and completion status
                match task.completed {
                    true => println!("[v] {}: {}", task.id, task.title),
                    false => println!("[ ] {}: {}", task.id, task.title),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager::new();

        manager.add(String::from("Learn Rust"));

        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.tasks[0].title, "Learn Rust");
        assert!(!manager.tasks[0].completed);
        assert_eq!(manager.next_id, 1);
    }

    #[test]
    fn test_add_multiple_tasks() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        manager.add(String::from("Task A"));
        manager.add(String::from("Task B"));
        manager.add(String::from("Task C"));

        assert_eq!(manager.tasks.len(), 3);
        assert_eq!(manager.tasks[0].title, "Task A");
        assert_eq!(manager.tasks[1].title, "Task B");
        assert_eq!(manager.tasks[2].title, "Task C");
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.tasks[1].id, 1);
        assert_eq!(manager.tasks[2].id, 2);
        assert!(!manager.tasks[0].completed);
        assert!(!manager.tasks[1].completed);
        assert!(!manager.tasks[2].completed);
        assert_eq!(manager.next_id, 3);
    }

    #[test]
    fn test_complete_tasks() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        manager.add(String::from("ID 0"));
        manager.add(String::from("ID 1"));

        assert!(manager.complete(0));
        assert!(manager.complete(0)); // complete(complete(task)) = complete(task)
        assert!(manager.tasks[0].completed);
        assert!(!manager.tasks[1].completed);

        assert!(!manager.complete(999));
    }

    #[test]
    fn test_remove_after_index_shift() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        manager.add(String::from("Task A")); // ID 0
        manager.add(String::from("Task B")); // ID 1
        manager.add(String::from("Task C")); // ID 2

        assert!(manager.remove(1));
        assert!(manager.remove(2));

        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.next_id, 3);
    }

    #[test]
    fn test_remove_after_index_shift2() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        manager.add(String::from("Task A")); // ID 0
        manager.add(String::from("Task B")); // ID 1
        assert!(manager.remove(0));
        manager.add(String::from("Task C")); // ID 2

        assert_eq!(manager.tasks[0].id, 1);
        assert_eq!(manager.tasks[1].id, 2);
        assert_eq!(manager.next_id, 3);
    }

    #[test]
    fn test_remove_and_add_tasks() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        manager.add(String::from("Task A"));
        assert_unique_ids(&manager);
        manager.add(String::from("Task B"));
        assert_unique_ids(&manager);
        manager.add(String::from("Task C"));
        assert_unique_ids(&manager);

        // Step 1: Remove task ID 1
        assert!(manager.remove(1));
        assert_unique_ids(&manager);
        // Step 2: Verify remaining IDs are 0 and 2
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.tasks[1].id, 2);

        // Step 3: Add Task D
        manager.add(String::from("Task D"));
        assert_unique_ids(&manager);
        // Step 4: Verify the new task has ID 3
        assert_eq!(manager.tasks[2].id, 3);
        // Step 5: Remove task ID 2
        assert!(manager.remove(2));
        assert_unique_ids(&manager);
        // Step 6: Verify remaining IDs are 0 and 3
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.tasks[1].id, 3);
        // Step 7: Try removing ID 999 and verify
        assert!(!manager.remove(999));
        assert_unique_ids(&manager);
        //         nothing changes
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[0].id, 0);
        assert_eq!(manager.tasks[1].id, 3);
        assert_eq!(manager.next_id, 4);
    }

    #[test]
    fn test_many_operations() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 0,
        };

        // Step 1: Add 100 tasks
        for i in 0..100 {
            manager.add(format!("Task {}", i));
            assert_unique_ids(&manager);
        }

        // Step 2: Remove all even-numbered task IDs
        for id in (0..100).step_by(2) {
            // TODO: Remove the task
            // TODO: Check the unique ID invariant
            assert!(manager.remove(id), "Failed to remove task ID: {}", id);
            assert_unique_ids(&manager);
        }

        // Step 3: Add another 50 tasks
        for i in 100..150 {
            // TODO: Add a task
            // TODO: Check the unique ID invariant
            manager.add(format!("Task {}", i));
            assert_unique_ids(&manager);
        }

        // Step 4: Verify the final state
        // TODO: Check task count
        // TODO: Check next_id
        // TODO: Verify no existing tasks share an ID
        assert_eq!(manager.tasks.len(), 100);
        assert_eq!(manager.next_id, 150);
        assert_unique_ids(&manager);

        let actual_ids: HashSet<u32> = manager.tasks.iter().map(|task| task.id).collect();

        let expected_ids: HashSet<u32> = (1..100).step_by(2).chain(100..150).collect();

        assert_eq!(actual_ids, expected_ids);
    }

    #[test]
    fn test_add_overflow() {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: u32::MAX,
        };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            manager.add(String::from("Overflow Task"));
        }));

        // Verify that the operation panicked.
        assert!(result.is_err());

        // Verify that no task was inserted.
        assert_eq!(manager.tasks.len(), 0);

        // Verify that next_id remains unchanged.
        assert_eq!(manager.next_id, u32::MAX);
    }

    use std::collections::HashSet;

    fn assert_unique_ids(manager: &TaskManager) {
        let mut seen = HashSet::new();

        for task in manager.tasks.iter() {
            // TODO:
            // Insert the task ID into seen.
            // Assert that the insertion returns true.
            // If insertion returns false, a duplicate exists.
            assert!(seen.insert(task.id), "Duplicate task ID found: {}", task.id);
        }
    }
}
