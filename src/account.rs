use std::{error::Error, fmt};
use std::fmt::Formatter;
use std::path::Display;
use chrono::{Local, NaiveDate};
use sorted_list::SortedList;

pub struct Account {
    name: String,
    balance: f32,
    transactions: SortedList<NaiveDate, Transaction>,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: String::from(name),
            balance: 0f32,
            transactions: SortedList::new(),
        }
    }

    pub fn add_transaction(&mut self, label: &str, amount: f32)
    -> Result<(), TransactionCreationError>{
        let now = NaiveDate::from(Local::now().naive_local());
        self.balance += amount;
        self.transactions.insert(
            now,
            Transaction::new(label, amount, now)?
        );
        Ok(())
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
    amount: f32,
    time: NaiveDate,
}

impl Transaction {
    fn new(label: &str, amount: f32, now: NaiveDate)
        -> Result<Transaction, TransactionCreationError> {
        if label.is_empty() {
            return Err(TransactionCreationError::new("No name provided for transaction"));
        }
        Ok(Transaction {
            label: String::from(label),
            amount,
            time: now,
        })
    }

    pub fn edit_name(&mut self, new: &str) {
        self.label = String::from(new);
    }

    pub fn edit_amount(&mut self, new: f32) {
        self.amount = new;
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Date: {:?} | Label: {} | Amount: ${:?}", self.time, self.label, self.amount)
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