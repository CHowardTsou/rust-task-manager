# Rust Task Manager

A small, interactive command-line task manager written in Rust. It keeps a list of tasks in memory and lets you add, list, complete and remove them from a numbered menu.

## Overview

### Running it

Requires a Rust toolchain that supports the 2024 edition (Rust 1.85 or newer). There are no third-party dependencies.

```bash
cargo run
```

The program shows a menu and reads one choice per line from standard input:

```
===== TASK MANAGER =====
1. Add task
2. List tasks
3. Complete task
4. Remove task
5. Exit
```

| Option | Prompt | Result |
| --- | --- | --- |
| 1. Add task | Task title | Adds an incomplete task with the next free ID. |
| 2. List tasks | — | Prints every task as `[ ] id: title` (open) or `[v] id: title` (done), or `No Tasks Found.` |
| 3. Complete task | Task ID | Marks the task done. Prints `Task Not Found` for an unknown ID. |
| 4. Remove task | Task ID | Deletes the task. Prints `Task Not Found` for an unknown ID. |
| 5. Exit | — | Quits the program. |

The program also exits when standard input ends (Ctrl-D, or piped input running out) at any prompt, so it can be scripted, for example `printf '1\nLearn Rust\n2\n' | cargo run`.

An ID that is not a non-negative integer gets `Please enter a valid task ID.`, and any other menu input gets `Invalid option`.

### Project layout

| File | Contents |
| --- | --- |
| `src/main.rs` | The menu loop and input handling (`read_line`, `read_task_id`). |
| `src/task_manager.rs` | The `Task` and `TaskManager` types and their unit tests. |

### Data model

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>, // kept in insertion order
    next_id: u32,     // the ID the next added task will get
}
```

`TaskManager` exposes four operations:

- `add(title: String)` creates a task with ID `next_id`, then increments `next_id`.
- `complete(id: u32) -> bool` marks a task done. It returns `false` if no task has that ID.
- `remove(id: u32) -> bool` deletes a task. It returns `false` if no task has that ID.
- `list(&self)` prints all tasks to standard output.

IDs start at `0` and are never reused. After you remove task `1`, the next task added still gets a fresh ID rather than `1`.

## Rust concepts

This project is a compact tour of several core Rust ideas:

- **Modules.** `mod task_manager;` in `main.rs` pulls in `src/task_manager.rs`. `use task_manager::TaskManager;` brings the type into scope.
- **Structs and visibility.** `Task` has `pub` fields. `TaskManager`'s fields are private, so code outside the module can change tasks only through its methods. This is what lets the type uphold its ID invariants (see below).
- **`impl` blocks and associated functions.** `TaskManager::new()` is a constructor-style associated function that returns `Self`. The other methods take `self` by reference.
- **Borrowing: `&self` vs `&mut self`.** `list` only reads, so it takes `&self`. `add`, `complete` and `remove` change state, so they take `&mut self`. The borrow checker therefore lets `main` call them only through the mutable binding `let mut manager`.
- **Ownership transfer.** `add` takes `title: String` by value, so the caller's string moves into the new `Task` and is not copied. `main` builds it with `title.trim().to_string()`, which turns a borrowed `&str` slice into an owned `String`.
- **`Option<T>` instead of sentinel values.** `read_line` returns `Option<String>`, with `None` meaning input has ended. `read_task_id` returns `Option<Option<u32>>`: the outer `None` means end of input, `Some(None)` means the line was not a valid ID. The caller has to handle all three cases with `match`, and `let Some(input) = read_line() else { break };` (a `let`-`else` statement) leaves the loop on end of input.
- **`Result<T, E>` and parsing.** `read_line` matches on `io::stdin().read_line(...)`, treating `Ok(0)` as end of input and `Err` as a reported failure instead of panicking. `str::parse::<u32>()` returns a `Result` that `.ok()` turns into an `Option`, so negative numbers, letters and values above `u32::MAX` all become `None`.
- **Checked arithmetic.** `next_id.checked_add(1)` returns `None` on overflow instead of wrapping around silently (as release builds would do with a plain `+`).
- **Iterators and closures.** `complete` walks `iter_mut()` to change a task in place. `remove` uses `iter().position(|x| x.id == id)` to find an index with a closure, then `Vec::remove`, which keeps the remaining tasks in order.
- **Exhaustive `match`.** The menu dispatches on `input.trim()` with a `_` catch-all arm, and `list` matches on the `bool` `completed`.
- **Unit tests in the same file.** The `#[cfg(test)] mod test` block is compiled only for `cargo test`. Because it is a child module, `use super::*;` gives it access to the private `tasks` and `next_id` fields.
- **Panic testing.** `std::panic::catch_unwind` with `AssertUnwindSafe` lets a test check that `add` panics on overflow *and* leaves the manager unchanged afterwards.

## Security properties

The program has no network access, no file I/O, no `unsafe` code and no dependencies outside the standard library. Its attack surface is the text typed on standard input. Within that, it guarantees the following:

- **Memory safety.** The code is entirely safe Rust, so buffer overflows, use-after-free and data races cannot happen.
- **Unique IDs.** Every task in the list has a distinct ID at all times. IDs come from a counter that only goes up, and `TaskManager`'s fields are private, so no outside code can insert a task with a duplicate ID or wind the counter back.
- **No ID reuse.** A removed task's ID is never handed out again. So an ID you saw earlier can never silently refer to a *different* task, and a stale "complete 3" or "remove 3" cannot act on an unrelated, newer task.
- **No silent integer overflow.** `add` checks the increment *before* changing any state. If `next_id` would overflow, it panics with `Task ID overflow`, and it does so before pushing the task. The list and counter are left exactly as they were, rather than wrapping to `0` and creating duplicate IDs.
- **Strict ID parsing.** IDs are parsed as `u32`. Negative, non-numeric and out-of-range input is rejected with a message instead of being coerced into some other ID.
- **Operations on missing IDs are harmless.** `complete` and `remove` return `false` and change nothing when the ID does not exist.
- **Titles are treated as data.** Task titles are only stored and printed with `{}` formatting. They are never interpreted as format strings, commands or paths.

## Testing

Run the unit tests with:

```bash
cargo test
```

All 8 tests live in `src/task_manager.rs`:

| Test | What it checks |
| --- | --- |
| `test_add_task` | A single add produces ID `0`, the right title, `completed == false`, and `next_id == 1`. |
| `test_add_multiple_tasks` | Sequential adds get IDs `0, 1, 2` in insertion order, and `next_id` advances to `3`. |
| `test_complete_tasks` | Completing is idempotent, affects only the target task, and returns `false` for an unknown ID. |
| `test_remove_after_index_shift` | Removing by ID still works after earlier removals have shifted vector indices. |
| `test_remove_after_index_shift2` | A task added after a removal gets a new ID and does not reuse the removed one. |
| `test_remove_and_add_tasks` | A scripted mix of adds and removes keeps the expected IDs, and removing a missing ID changes nothing. |
| `test_many_operations` | Stress test: add 100, remove the 50 even IDs, add 50 more. Checks the count (100), `next_id` (150) and the exact final ID set. |
| `test_add_overflow` | With `next_id == u32::MAX`, `add` panics, inserts no task and leaves `next_id` unchanged. |

Most tests call the helper `assert_unique_ids` after every change. It inserts each ID into a `HashSet` and fails on the first duplicate, which checks the uniqueness invariant continuously rather than only at the end.

The tests cover `TaskManager`'s logic only. The interactive menu in `main.rs` and the output of `list` (which prints straight to stdout) have no automated tests.

## Limitations

- **No persistence.** Tasks live only in memory and are lost when the program exits.
- **Unreadable input ends the session.** A failed read, such as input that is not valid UTF-8, prints `Failed to read input: ...` to stderr and exits rather than letting you retry. (End of input, from Ctrl-D or piped input running out, also exits cleanly.)
- **Empty titles are accepted.** A blank line at the title prompt creates a task with an empty title. There is also no length limit.
- **Finite ID space.** IDs are `u32` and never reused. Because the overflow check runs before insertion, ID `u32::MAX` itself is never issued, so the program can create at most 4,294,967,295 tasks over one run before `add` panics.
- **Linear-time lookups.** `complete` and `remove` scan the whole `Vec`, and `remove` shifts every later element. That is fine at interactive scale, but a `HashMap<u32, Task>` or `BTreeMap` would suit very large lists better.
- **No editing or undo.** You cannot rename a task, mark it incomplete again, or restore a removed task.
- **`list` prints rather than returns.** Since `list` writes straight to stdout, its output cannot be reused or unit-tested. Returning the tasks (or implementing `Display`) would fix both.
- **Single user, single thread.** `TaskManager` has no synchronization and is meant to be used from the one interactive loop.
