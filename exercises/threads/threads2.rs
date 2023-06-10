// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // let status = Arc::new(JobStatus { jobs_completed: 0 })
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut num = status_shared.lock().unwrap();
            // TODO: You must take an action before you update a shared value
            // status_shared.jobs_completed += 1
            num.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // println!("jobs completed {}", status.jobs_completed);
        // When printing each handle index i, more than i+1 threads may be finished,
        // so jobs completed y (y > i + 1) can be printed.
        // However, since handles is generated from index 0 to end,
        // so y must increase(or remain the same) when i increase,
        // because:
        // when printing at index i:     [?, ?, ..., ?, finished i, ?, ?, ...]
        // when printing at index i + 1: [?', ?', ..., ?', finished i, finished i+1, ?', ...]
        // Here, when we move from i to i+1, threads that are already finished doesn't affect to
        // the count, whereas threads that are not finished when we call index i may be finished
        // when we call index i + 1. Also, that count may not increase when i+1 is already finished
        // where we call index i.
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
