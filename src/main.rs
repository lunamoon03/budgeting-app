use std::{error::Error, fmt};
use std::fmt::Formatter;
use chrono::{DateTime, Local};

fn main() {}


struct Transaction {
    label: String,
    amount: i32,
    time: DateTime<Local>,
}

impl Transaction {
    pub fn new(label: &str, amount: i32) -> Result<Transaction, TransactionCreationError> {
        if label.is_empty() {
            return Err(TransactionCreationError::new("No name provided for transaction"));
        }
        Ok(Transaction {
            label: String::from(label),
            amount,
            time: Local::now(),
        })
    }
}

#[derive(Debug)]
struct TransactionCreationError {
    reason: String,
}

impl TransactionCreationError {
    pub fn new(reason: &str) -> TransactionCreationError{
        TransactionCreationError {
            reason: String::from(reason),
        }
    }
}

impl Error for TransactionCreationError {}

impl fmt::Display for TransactionCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
