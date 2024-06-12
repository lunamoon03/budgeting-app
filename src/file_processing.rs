use crate::account::Account;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::str::FromStr;

pub(super) fn get_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => fs::File::create_new(file_path)?, // if file cannot be opened it does not exist
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub(super) fn write_to_file(
    file_path: &str,
    accounts: HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    let mut buf: Vec<String> = Vec::new();
    for account in accounts.values() {
        buf.push(format!("{}{{", account.name()));

        for (_, transaction) in account.transactions().iter() {
            buf.push(format!("{}", transaction.amount()));
            buf.push(transaction.label().to_string());
            buf.push(format!("{}", transaction.date()));
        }
        buf.push(String::from("}"));
    }

    if let Err(e) = fs::write(file_path, buf.join(",")) {
        return Err(Box::from(format!(
            "Failed writing to {}:\n{}",
            file_path, e
        )));
    }

    Ok(())
}

pub(super) fn read_from_string(contents: String) -> Result<HashMap<String, Account>, Box<dyn Error>> {
    if contents.is_empty() {
        return Ok(HashMap::new());
    }

    let split: Vec<&str> = contents.split(',').collect();

    if !split.first().unwrap().ends_with('{') || split.len() < 2 {
        return Err(Box::from("Malformed File - Comma Separators"));
    }

    let mut accounts: HashMap<String, Account> = HashMap::new();
    let mut account_holder: Account;
    let mut iter = 0;
    while iter < split.len() {
        (account_holder, iter) = read_account(&split, iter)?;

        accounts.insert(String::from(account_holder.name()), account_holder);
    }

    Ok(accounts)
}

fn read_account(split: &[&str], mut iter: usize) -> Result<(Account, usize), Box<dyn Error>> {
    // unwrap safe as we know that iter is at most at very last spot at start of loop

    let mut account = Account::new(split.get(iter).unwrap().trim_end_matches('{').trim());

    iter += 1; // transaction amount OR "}"

    while iter < split.len() - 2 && !&split[iter].eq("}") {
        let transaction_amount: f32 = match split.get(iter) {
            Some(slice) => match slice.parse() {
                Ok(val) => val,
                _ => return Err(Box::from(format!("Value {} not valid", slice))),
            },
            _ => break,
        };

        iter += 1; // transaction nameq

        let transaction_name = match split.get(iter) {
            Some(slice) => slice,
            _ => break,
        };

        iter += 1; // transaction date

        let transaction_date: NaiveDate = match split.get(iter) {
            Some(slice) => match NaiveDate::from_str(slice) {
                Ok(val) => val,
                _ => return Err(Box::from(format!("Date {} not valid", slice))),
            },
            _ => break,
        };

        iter += 1; // next token (should be "}")

        if let Err(e) =
            account.add_transaction(transaction_name, transaction_amount, transaction_date)
        {
            return Err(Box::from(e));
        }
    }
    if iter >= split.len() {
        return Err(Box::from("Malformed file - Wrong number of entries"))
    }
    if !&split[iter].eq("}") {
        return Err(Box::from("Malformed file - Ending Braces"));
    }

    iter += 1;
    Ok((account, iter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, NaiveDate};

    #[test]
    fn empty_file() {
        let empty = get_file_contents("src/test-files/empty.csv").unwrap();
        assert!(read_from_string(empty).unwrap().is_empty());
    }

    #[test]
    #[should_panic]
    fn bad_start() {
        let _ = read_from_string(get_file_contents("src/test-files/bad-start.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_end() {
        let _ = read_from_string(get_file_contents("src/test-files/bad-end.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_trans_label() {
        let _ = read_from_string(get_file_contents("src/test-files/bad-trans-label.csv").unwrap())
            .unwrap();
    }

    #[test]
    fn one_account() {
        let a = get_file_contents("src/test-files/1-account.csv").unwrap();
        assert_eq!(
            &format!("{}", read_from_string(a).unwrap().get("Savings").unwrap()),
            "Name: Savings | Balance: $1.00\n\
                   Transactions:\n\
                   Date: 10 Jun 2024 | Label: a | Amount: $1.00"
        );
    }

    #[test]
    fn two_accounts() {
        let b = read_from_string(get_file_contents("src/test-files/2-account.csv").unwrap()).unwrap();
        assert_eq!(
            &format!("{}", b.get("Savings").unwrap()),
            "Name: Savings | Balance: $1.00\n\
                   Transactions:\n\
                   Date: 10 Jun 2024 | Label: a | Amount: $1.00"
        );
        assert_eq!(
            &format!("{}", b.get("Expenses").unwrap()),
            "Name: Expenses | Balance: $3.00\n\
                   Transactions:\n\
                   Date: 10 Jun 2024 | Label: c | Amount: $3.00"
        );
    }

    fn form_account() -> Account {
        let mut a = Account::new("Savings");
        a.add_new_transaction("a", 1.0).unwrap();
        a.add_new_transaction("b", 2.0).unwrap();
        a
    }

    #[test]
    fn write_test() {
        let account = form_account();
        let map = HashMap::from([(String::from(account.name()), account)]);

        let file_path = "src/test-files/write-test";

        write_to_file(file_path, map).unwrap();

        let file_contents = get_file_contents(file_path).unwrap();

        let date = NaiveDate::from(Local::now().naive_local());

        assert_eq!(
            &file_contents,
            &format!("Savings{{,1,a,{date},2,b,{date},}}")
        );
    }

    #[test]
    fn write_read_test() {
        let file_path = "src/test-files/write-read-test";

        let account = form_account();
        let map = HashMap::from([(String::from(account.name()), account)]);

        write_to_file(file_path, map).unwrap();

        let file_contents = get_file_contents(file_path).unwrap();

        let binding = read_from_string(file_contents).unwrap();
        let account2 = binding.get("Savings").unwrap();

        assert_eq!(&format!("{}", form_account()), &format!("{account2}"));
    }
}
