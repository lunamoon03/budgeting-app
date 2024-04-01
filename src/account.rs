use std::{error::Error, fmt};
use std::fmt::Formatter;
use chrono::{Local, NaiveDate};
use sorted_list::SortedList;
use itertools::Itertools;
use rust_decimal::Decimal;
use rusty_money::{Money, iso::Currency, iso};

pub struct Account<'a> {
    name: String,
    balance: Money<'a, Currency>,
    transactions: SortedList<NaiveDate, Transaction<'a>>,
}

impl<'a> Account<'a> {
    pub fn new(name: &str) -> Account {
        Account {
            name: String::from(name),
            balance: Money::from_decimal(Decimal::from(0), iso::NZD),
            transactions: SortedList::new(),
        }
    }

    pub fn add_transaction(&'a mut self, label: &'a str, amount: Decimal)
                           -> Result<(), TransactionCreationError> {
        let now = NaiveDate::from(Local::now().naive_local());
        //self.balance += amount;
        self.transactions.insert(
            now,
            Transaction::new(label, amount, now)?
        );
        Ok(())
    }
}

impl fmt::Display for Account<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {} | Balance: ${:?}\nTransactions:\n{}",
            self.name,
            self.balance,
            self.transactions.values().join("\n"),
        )
    }
}

#[derive(PartialEq)]
struct Transaction<'a> {
    label: String,
    amount: Money<'a, Currency>,
    time: NaiveDate,
}

impl Transaction<'_> {
    fn new(label: &str, amount: Decimal, now: NaiveDate)
            -> Result<Transaction, TransactionCreationError> {
        if label.is_empty() {
            return Err(TransactionCreationError::new("No name provided for transaction"));
        }
        Ok(Transaction {
            label: String::from(label),
            amount: Money::from_decimal(amount, iso::NZD),
            time: now,
        })
    }

    pub fn _edit_name(&mut self, new: &str) {
        self.label = String::from(new);
    }

    pub fn _edit_amount(&mut self, new: Decimal) {
        self.amount = Money::from_decimal(new, iso::NZD);
    }
}

impl fmt::Display for Transaction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Date: {:?} | Label: {} | Amount: ${:?}", self.time, self.label, self.amount)
    }
}

#[derive(Debug)]
pub struct TransactionCreationError {
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

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};
    use rust_decimal::Decimal;
    use crate::account::Account;
    #[test]
    fn balance_increasing() {
        let mut a = Account::new("");
        let mut sum: f32 = 0.0;
        for i in 0..100 {
            a.add_transaction(&format!("{i}"), Decimal::from(i)).unwrap();
            sum += i as f32;
        }
        assert_eq!(a.balance, sum);
    }

    #[test]
    fn balance_decreasing() {
        let mut a = Account::new("");
        a.add_transaction("x", Decimal::from(100)).unwrap();
        assert_eq!(a.balance, Decimal::from(100));
        a.add_transaction("y", Decimal::from(-50)).unwrap();
        assert_eq!(a.balance, Decimal::from(50));
    }

    #[test]
    #[should_panic]
    fn transaction_add_error() {
        let mut a = Account::new("");
        a.add_transaction("", Decimal::from(15)).unwrap();
    }

    #[test]
    fn account_printing() {
        let mut a = Account::new("Savings");
        a.add_transaction("Transaction", Decimal::from(10)).unwrap();
        assert_eq!(&format!("{}", a),
                   &format!("Name: Savings | Balance: $10.0\n\
                   Transactions:\n\
                   Date: {:?} | Label: Transaction | Amount: $10.0",
                            NaiveDate::from(Local::now().naive_local())));
    }
}