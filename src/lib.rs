use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use chrono::NaiveDate;
use crate::account::Account;

mod account;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = get_file_contents(file_path)?;

    let _accounts = process_file(contents);

    Ok(())
}

pub fn parse_args(args: &[String]) -> Result<&str, Box<dyn Error>> {
    let file_type = ".csv";
    if args.len()!=2 {
        return Err(Box::from(format!("Enter only one argument: [filename{}]",file_type)));
    };

    let file_path = match args.get(1) {
        Some(file_path) if file_path.ends_with(file_type) => file_path,
        _ => return Err(Box::from(format!("Enter a valid file name: [filename{}]",file_type))),
    };

    Ok(file_path)
}

fn get_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => File::create_new(file_path)?, // if file cannot be opened it does not exist
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_file(contents: String) -> Result<Vec<Account>, Box<dyn Error>> {
    if contents.is_empty() {
        return Ok(Vec::new());
    }

    let split: Vec<&str> = contents.split(",").collect();

    if !split.get(0).unwrap().ends_with("{") || split.len()<2 {
        return Err(Box::from("Malformed File"));
    }

    let mut accounts: Vec<Account> = Vec::new();
    let mut account_holder: Account;
    let mut iter = 0;
    while iter < split.len()  {
        (account_holder,iter) = process_account(&split, iter)?;

        accounts.push(account_holder);
    }

    Ok(accounts)
}

fn process_account(split: &Vec<&str>, mut iter: usize) -> Result<(Account,usize), Box<dyn Error>> {
// unwrap safe as we know that iter is at most at very last spot at start of loop

    let mut account = Account::new(split.get(iter).unwrap().trim_end_matches('{').trim());

    iter += 1; // transaction amount OR "}"

    while iter < split.len() - 2 && !&split[iter].eq("}") {
        let transaction_amount: f32 = match split.get(iter) {
            Some(slice) => match slice.parse() {
                Ok(val) => val,
                _ => return Err(Box::from(format!("Value {} not valid", slice)))
            },
            _ => break
        };

        iter += 1; // transaction name

        let transaction_name = match split.get(iter) {
            Some(slice) => slice,
            _ => break
        };

        iter += 1;

        let transaction_date: NaiveDate = match split.get(iter) {
            Some(slice) => match NaiveDate::from_str(slice) {
                Ok(val) => val,
                _ => return Err(Box::from(format!("Date {} not valid", slice)))
            },
            _ => break
        };

        iter += 1; // next token (should be "}")

        if let Err(e) = account.add_transaction(
            transaction_name, transaction_amount, transaction_date) {
            return Err(Box::from(e))
        }
    }
    if !&split[iter].eq("}") {
        return Err(Box::from("Malformed file"))
    }

    iter += 1;
    Ok((account, iter))
}


fn write_to_file(file_path: &str, accounts: Vec<Account>) -> Result<(), Box<dyn Error>> {
    let mut buf: Vec<String> = Vec::new();
    for account in accounts.iter() {
        buf.push(format!("{}{{",account.name()));

        for (_,transaction) in account.transactions().iter() {
            buf.push(format!("{}", transaction.amount()));
            buf.push(format!("{}", transaction.label()));
            buf.push(format!("{}", transaction.date()));
        }
        buf.push(String::from("}"));
    }

    if let Err(e) = fs::write(file_path, buf.join(",")) {
        return Err(Box::from(format!("Failed writing to {}:\n{}",file_path,e)))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};
    use super::*;

    #[test]
    fn empty_file() {
        let empty = get_file_contents("src/test-files/empty.csv").unwrap();
        assert!(process_file(empty).unwrap().is_empty());
    }

    #[test]
    #[should_panic]
    fn bad_start() {
        let _ = process_file(get_file_contents("src/test-files/bad-start.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_end() {
        let _ = process_file(get_file_contents("src/test-files/bad-end.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_trans_label() {
        let _ = process_file(get_file_contents("src/test-files/bad-trans-label.csv").unwrap()).unwrap();
    }

    #[test]
    fn one_account() {
        let a = get_file_contents("src/test-files/1-account.csv").unwrap();
        assert_eq!(&format!("{}", process_file(a).unwrap().get(0).unwrap()),
                   "Name: Savings | Balance: $1.0\n\
                   Transactions:\n\
                   Date: 2024-06-10 | Label: a | Amount: $1.0");
    }

    #[test]
    fn two_accounts() {
        let b = process_file(get_file_contents("src/test-files/2-account.csv").unwrap()).unwrap();
        assert_eq!(&format!("{}", b.get(0).unwrap()),
                   "Name: Savings | Balance: $1.0\n\
                   Transactions:\n\
                   Date: 2024-06-10 | Label: a | Amount: $1.0");
        assert_eq!(&format!("{}", b.get(1).unwrap()),
                   "Name: Expenses | Balance: $3.0\n\
                   Transactions:\n\
                   Date: 2024-06-10 | Label: c | Amount: $3.0");
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

        let file_path = "src/test-files/write-test";

        write_to_file(file_path, vec!(account)).unwrap();

        let file_contents = get_file_contents(file_path).unwrap();

        let date = NaiveDate::from(Local::now().naive_local());

        assert_eq!(&file_contents,
                   &format!("Savings{{,1,a,{date},2,b,{date},}}"));
    }

    #[test]
    fn write_read_test() {
        let file_path = "src/test-files/write-read-test";

        write_to_file(file_path, vec!(form_account())).unwrap();

        let file_contents = get_file_contents(file_path).unwrap();

        let binding = process_file(file_contents).unwrap();
        let account2 = binding.get(0).unwrap();

        assert_eq!(&format!("{}", form_account()),
                   &format!("{account2}"));
    }
}