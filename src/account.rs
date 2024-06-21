use chrono::{Local, NaiveDate};
use itertools::Itertools;
use std::fmt::Formatter;
use std::{error::Error, fmt};
use std::collections::HashMap;

pub struct Account {
    name: String,
    balance: f32,
    transactions: HashMap<String, Transaction>,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: String::from(name),
            balance: 0.0,
            transactions: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn balance(&self) -> &f32 {
        &self.balance
    }

    pub fn transactions(&self) -> &HashMap<String, Transaction> {
        &self.transactions
    }

    pub fn add_new_transaction(
        &mut self,
        label: &str,
        amount: f32,
    ) -> Result<(), TransactionCreationError> {
        let today = NaiveDate::from(Local::now().naive_local());
        self.balance += amount;
        self.transactions
            .insert(format!("{}-{}-{}", today, label, amount), Transaction::new(label, amount, today)?);
        Ok(())
    }

    pub fn add_transaction(
        &mut self,
        label: &str,
        amount: f32,
        date: NaiveDate,
    ) -> Result<(), TransactionCreationError> {
        self.balance += amount;
        self.transactions
            .insert(format!("{}-{}-{}",date,label,amount), Transaction::new(label, amount, date)?);
        Ok(())
    }

    pub fn remove_transaction(
        &mut self,
        key: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !self.transactions.contains_key(key) {
            return Err(Box::from(format!(
                "Transaction {} is not present in the account",
                key
            )))
        }
        self.transactions.remove(key);
        Ok(())
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {} | Balance: ${:.2}\nTransactions:\n{}",
            self.name,
            self.balance,
            self.transactions
                .values()
                .sorted_by(|a, b| Ord::cmp(&a.date, &b.date))
                .join("\n"),
        )
    }
}

#[derive(PartialEq)]
pub struct Transaction {
    label: String,
    amount: f32,
    date: NaiveDate,
}

impl Transaction {
    fn new(
        label: &str,
        amount: f32,
        now: NaiveDate,
    ) -> Result<Transaction, TransactionCreationError> {
        if label.is_empty() {
            return Err(TransactionCreationError::new(
                "No name provided for transaction",
            ));
        }
        Ok(Transaction {
            label: String::from(label),
            amount,
            date: now,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn amount(&self) -> &f32 {
        &self.amount
    }
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn _edit_name(&mut self, new: &str) {
        self.label = String::from(new);
    }

    pub fn _edit_amount(&mut self, new: f32) {
        self.amount = new;
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {} | Label: {} | Amount: ${:.2}",
            self.date.format("%d %b %Y"),
            self.label,
            self.amount
        )
    }
}

#[derive(Debug)]
pub struct TransactionCreationError {
    reason: String,
}

impl TransactionCreationError {
    pub fn new(reason: &str) -> TransactionCreationError {
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

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use chrono::{Local, NaiveDate};
    #[test]
    fn balance_increasing() {
        let mut a = Account::new("");
        let mut sum: f32 = 0.0;
        for i in 0..100 {
            a.add_new_transaction(&format!("{i}"), i as f32).unwrap();
            sum += i as f32;
        }
        assert_eq!(a.balance, sum);
    }

    #[test]
    fn balance_decreasing() {
        let mut a = Account::new("");
        a.add_new_transaction("x", 100.0).unwrap();
        assert_eq!(a.balance, 100.0);
        a.add_new_transaction("y", -50.0).unwrap();
        assert_eq!(a.balance, 50.0);
    }

    #[test]
    #[should_panic]
    fn transaction_add_error() {
        let mut a = Account::new("");
        a.add_new_transaction("", 15.0).unwrap();
    }

    #[test]
    fn account_printing() {
        let mut a = Account::new("Savings");
        a.add_new_transaction("Transaction", 10.0).unwrap();
        assert_eq!(
            &format!("{}", a),
            &format!(
                "Name: Savings | Balance: $10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction | Amount: $10.00",
                NaiveDate::from(Local::now().naive_local()).format("%d %b %Y")
            )
        );
    }
}
