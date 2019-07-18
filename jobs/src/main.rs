extern crate schedule;
extern crate chrono;
extern crate hero_lib;

use schedule::{Agenda, Job};
use chrono::prelude::*;

use hero_lib::dbs::redis::pool;
use hero_lib::settings::Settings;
use r2d2_redis::redis;
use std::ops::Deref;

fn main() {

    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let pool = pool(settings.redis);
    let conn = pool.get().unwrap();

    let reply = redis::cmd("PING").query::<String>(conn.deref()).unwrap();
    // Alternatively, without deref():
    // let reply = redis::cmd("PING").query::<String>(&*conn).unwrap();
    println!("{:?}", reply);
    assert_eq!("PONG", reply);

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