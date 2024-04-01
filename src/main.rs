use std::fmt::Error;
use chrono::{DateTime, Local};

fn main() {
    println!("Hello, world!");
}


struct Transaction {
    label: String,
    amount: i32,
    time: DateTime<Local>,
}

impl Transaction {
    pub fn new(label: &str, amount: i32) -> Result<Transaction, Error> {
        Ok(Transaction {
            label: String::from(label),
            amount,
            time: Local::now(),
        })
    }
}