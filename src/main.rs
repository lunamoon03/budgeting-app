use std::{error::Error, fmt};
use std::fmt::Formatter;
use chrono::{Local, NaiveDate};
use sorted_list::SortedList;

fn main() {
}


struct Account {
    name: String,
    balance: i32,
    transactions: SortedList<NaiveDate, Transaction>,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: String::from(name),
            balance: 0,
            transactions: SortedList::new(),
        }
    }

    pub fn add_transaction(&mut self, t: Transaction) {
        self.balance += t.amount;
        self.transactions.insert(t.time, t);
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(PartialEq)]
struct Transaction {
    label: String,
    amount: i32,
    time: NaiveDate,
}

impl Transaction {
    pub fn new(label: &str, amount: i32) -> Result<Transaction, TransactionCreationError> {
        if label.is_empty() {
            return Err(TransactionCreationError::new("No name provided for transaction"));
        }
        Ok(Transaction {
            label: String::from(label),
            amount,
            time: NaiveDate::from(Local::now().naive_local()),
        })
    }

    pub fn edit_name(&mut self, new: &str) {
        self.label = String::from(new);
    }

    pub fn edit_amount(&mut self, new: i32) {
        self.amount = new;
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
