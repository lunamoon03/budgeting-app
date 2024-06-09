use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::account::Account;

mod account;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = get_file_contents(file_path)?;

    let _accounts = read_file(contents);

    Ok(())
}

pub fn parse_args(args: &[String]) -> Result<&str, Box<dyn Error>> {
    let file_type = ".csv";
    if args.len()!=2 {
        return Err(Box::from("Enter only one argument: [filename.csv]"));
    };

    let file_path = match args.get(1) {
        Some(file_path) if file_path.ends_with(file_type) => file_path,
        _ => return Err(Box::from("Enter a valid filename: [filename.csv]")),
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

fn read_file(contents: String) -> Result<Vec<Account>, Box<dyn Error>> {
    if contents.is_empty() {
        return Ok(Vec::new());
    }

    let split: Vec<&str> = contents.split(",").collect();

    if !split.get(0).unwrap().ends_with("{") || split.len() % 2 != 0 {
        return Err(Box::from("Malformed File"));
    }

    let mut accounts: Vec<Account> = Vec::new();
    let mut iter = 0;
    while iter < split.len()  {

        // unwrap safe as we know that iter is at most at very last spot at start of loop
        let account_name = split.get(iter).unwrap().trim_end_matches("{").trim();

        let mut account = Account::new(account_name);

        iter += 1; // transaction amount OR "}"

        while iter < split.len()-2 && !&split[iter].eq("}") {

            let transaction_amount: f32 = match split.get(iter) {
                Some(slice) => match slice.parse() {
                    Ok(val) => val,
                    _ => return Err(Box::from(format!("Value {} not valid", slice)))
                },
                _ => break
            };

            iter += 1; // transaction name

            let transaction_name = match split.get(iter){
                Some(slice) => slice,
                _ => break
            };

            iter += 1; // next token (should be "}")

            if let Err(e) = account.add_transaction(transaction_name, transaction_amount) {
                return Err(e.into())
            }
        }
        if !&split[iter].eq("}") {
            return Err(Box::from("Malformed file"))
        }

        iter +=1;

        accounts.push(account);
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};
    use super::*;

    #[test]
    fn empty_file() {
        let empty = get_file_contents("src/test-files/empty.csv").unwrap();
        assert!(read_file(empty).unwrap().is_empty());
    }

    #[test]
    #[should_panic]
    fn bad_start() {
        let _ = read_file(get_file_contents("src/test-files/bad-start.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_end() {
        let _ = read_file(get_file_contents("src/test-files/bad-end.csv").unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_trans_label() {
        let _ = read_file(get_file_contents("src/test-files/bad-trans-label.csv").unwrap()).unwrap();
    }

    #[test]
    fn one_account() {
        let a = get_file_contents("src/test-files/1-account.csv").unwrap();
        assert_eq!(&format!("{}", read_file(a).unwrap().get(0).unwrap()),
                   &format!("Name: Savings | Balance: $1.0\n\
                   Transactions:\n\
                   Date: {:?} | Label: a | Amount: $1.0",
                            NaiveDate::from(Local::now().naive_local())));
    }

    #[test]
    fn two_accounts() {
        let b = read_file(get_file_contents("src/test-files/2-account.csv").unwrap()).unwrap();
        assert_eq!(&format!("{}", b.get(0).unwrap()),
                   &format!("Name: Savings | Balance: $1.0\n\
                   Transactions:\n\
                   Date: {:?} | Label: a | Amount: $1.0",
                            NaiveDate::from(Local::now().naive_local())));
        assert_eq!(&format!("{}", b.get(1).unwrap()),
                   &format!("Name: Expenses | Balance: $3.0\n\
                   Transactions:\n\
                   Date: {:?} | Label: c | Amount: $3.0",
                            NaiveDate::from(Local::now().naive_local())));
    }
}