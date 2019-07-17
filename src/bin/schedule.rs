extern crate schedule;
extern crate chrono;

use schedule::{Agenda, Job};
use chrono::prelude::*;

fn main() {
    let mut a = Agenda::new();

    // Run every second
    a.add(Job::new(|| {
        println!("at second     :: {}", Utc::now());
    }, "* * * * * *".parse().unwrap()));

    // Run every minute
    a.add(Job::new(|| {
        println!("at minute     :: {}", Utc::now());
    }, "* * * * *".parse().unwrap()));

    // Run every hour
    a.add(Job::new(|| {
        println!("at hour       :: {}", Utc::now());
    }, "0 * * * *".parse().unwrap()));

    // Check and run pending jobs in agenda every 500 milliseconds
    loop {
        a.run_pending();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}