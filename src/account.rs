use chrono::{Local, NaiveDate};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::{error::Error, fmt};

pub struct Account {
    name: String,
    balance: f32,
    transactions: HashMap<String, Transaction>,
}

impl Account {
    fn new(name: &str) -> Account {
        Account {
            name: String::from(name),
            balance: 0.0,
            transactions: HashMap::new(),
        }
    }

    pub fn build(name: &str) -> Result<Account, Box<dyn Error>> {
        if name.trim_end_matches('\n').is_empty() {
            return Err(Box::from("Account must have a name"));
        }
        Ok(Self::new(name))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn _balance(&self) -> &f32 {
        &self.balance
    }

    pub fn transactions(&self) -> &HashMap<String, Transaction> {
        &self.transactions
    }

    pub fn add_new_transaction(&mut self, label: &str, amount: f32) -> Result<(), Box<dyn Error>> {
        let today = NaiveDate::from(Local::now().naive_local());
        self.check_transaction_not_exists(&format!("{}-{}-{}", today, label, amount))?;

        self.balance += amount;
        self.transactions.insert(
            format!("{}-{}-{}", today, label, amount),
            Transaction::new(label, amount, today)?,
        );
        Ok(())
    }

    pub fn add_transaction(
        &mut self,
        label: &str,
        amount: f32,
        date: NaiveDate,
    ) -> Result<(), Box<dyn Error>> {
        self.check_transaction_not_exists(&format!("{}-{}-{}", date, label, amount))?;

        self.balance += amount;
        self.transactions.insert(
            format!("{}-{}-{}", date, label, amount),
            Transaction::new(label, amount, date)?,
        );
        Ok(())
    }

    pub fn edit_transaction_label(
        &mut self,
        key: &str,
        new_label: String,
    ) -> Result<(), Box<dyn Error>> {
        self.check_transaction_exists(key)?;

        let transaction = self.transactions.get(key).unwrap();

        self.check_transaction_not_exists(&format!(
            "{}-{}-{}",
            transaction.date, new_label, transaction.date
        ))?;

        self.transactions
            .get_mut(key)
            .unwrap()
            .edit_label(new_label);
        Ok(())
    }

    pub fn edit_transaction_amount(
        &mut self,
        key: &str,
        new_amount: f32,
    ) -> Result<(), Box<dyn Error>> {
        self.check_transaction_exists(key)?;

        let transaction = self.transactions.get(key).unwrap();

        self.check_transaction_not_exists(&format!(
            "{}-{}-{}",
            transaction.date, transaction.label, new_amount
        ))?;

        self.balance -= self.transactions.get(key).unwrap().amount;
        self.balance += new_amount;

        self.transactions
            .get_mut(key)
            .unwrap()
            .edit_amount(new_amount);
        Ok(())
    }

    pub fn edit_transaction_date(
        &mut self,
        key: &str,
        new_date: NaiveDate,
    ) -> Result<(), Box<dyn Error>> {
        self.check_transaction_exists(key)?;

        let transaction = self.transactions.get(key).unwrap();

        self.check_transaction_not_exists(&format!(
            "{}-{}-{}",
            new_date, transaction.label, transaction.amount
        ))?;

        self.transactions.get_mut(key).unwrap().edit_date(new_date);
        Ok(())
    }

    pub fn remove_transaction(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.check_transaction_exists(key)?;
        self.balance -= self.transactions.remove(key).unwrap().amount;
        Ok(())
    }

    fn check_transaction_exists(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        if !self.transactions.contains_key(key) {
            return Err(Box::from(format!(
                "Transaction {} is not present in {}",
                key, self.name
            )));
        }
        Ok(())
    }

    fn check_transaction_not_exists(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        if self.transactions.contains_key(key) {
            return Err(Box::from(format!(
                "Transaction {} is already present in {}",
                key, self.name
            )));
        }
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
    fn new(label: &str, amount: f32, now: NaiveDate) -> Result<Transaction, Box<dyn Error>> {
        if label.is_empty() {
            return Err("No name provided for transaction".into());
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

    fn edit_label(&mut self, new: String) {
        self.label = new;
    }

    fn edit_amount(&mut self, new: f32) {
        self.amount = new;
    }

    fn edit_date(&mut self, new: NaiveDate) {
        self.date = new;
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

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use chrono::{Local, NaiveDate};

    #[test]
    #[should_panic]
    fn no_name() {
        Account::build("\n").unwrap();
    }

    #[test]
    fn balance_increasing() {
        let mut a = Account::build("account").unwrap();
        let mut sum: f32 = 0.0;
        for i in 0..100 {
            a.add_new_transaction(&format!("{i}"), i as f32).unwrap();
            sum += i as f32;
        }
        assert_eq!(a.balance, sum);
    }

    #[test]
    fn balance_decreasing() {
        let mut a = Account::build("account").unwrap();
        a.add_new_transaction("x", 100.0).unwrap();
        assert_eq!(a.balance, 100.0);
        a.add_new_transaction("y", -50.0).unwrap();
        assert_eq!(a.balance, 50.0);
    }

    #[test]
    #[should_panic]
    fn transaction_add_error() {
        let mut a = Account::build("account").unwrap();
        a.add_new_transaction("", 15.0).unwrap();
    }

    #[test]
    fn account_printing() {
        let mut a = Account::build("Savings").unwrap();
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
