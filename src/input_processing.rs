use std::collections::HashMap;
use std::error::Error;
use chrono::NaiveDate;
use crate::account::Account;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(super) fn add_new_transaction(inputs: Vec<String>, accounts: &mut HashMap<String, Account>)
                                  -> Result<(), Box<dyn Error>> {
    if inputs.len() != 4 {
        return Err(Box::from(format!(
            "Wrong number of inputs. {} when it should be 3"
            ,inputs.len()-1
        )));
    }

    // Can unwrap freely due to check of input len
    let account_name = match get_account_name(&inputs, accounts) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = match get_transaction_amount(&inputs) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!(
            "Account name {account_name} invalid"
        ))),
    };

    account.add_new_transaction(&label, amount)?;

    Ok(())
}

pub(super) fn add_transaction (inputs: Vec<String>, accounts: &mut HashMap<String, Account>)
                               -> Result<(), Box<dyn Error>> {
    if inputs.len() != 5 {
        return Err(Box::from(format!(
            "Wrong number of inputs. {} when it should be 3"
            ,inputs.len()-1
        )));
    }

    // Can unwrap freely due to check of input len
    let account_name = match get_account_name(&inputs, accounts) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = match get_transaction_amount(&inputs) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let date: NaiveDate = match inputs.get(4).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Box::from(format!(
            "Date entered invalid: {}",
            e
        ))),
    };

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!(
            "Account name {account_name} invalid"
        ))),
    };

    account.add_transaction(&label, amount, date)?;

    Ok(())
}



fn get_account_name(inputs: &[String], accounts: &HashMap<String, Account>)
    -> Result<String, Result<(), Box<dyn Error>>>
{
    let account_name = inputs.get(1).unwrap().to_lowercase();
    if !accounts.keys().contains(&account_name) {
        return Err(Err(Box::from(format!(
            "Account name {} not present.",
            account_name.to_case(Case::Title)
        ))))
    }
    Ok(account_name)
}

fn get_transaction_amount(inputs: &[String]) -> Result<f32, Result<(), Box<dyn Error>>> {
    let amount: f32 = match inputs.get(3).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Err(Box::from(format!(
            "Amount entered invalid: {}",
            e
        )))),
    };
    Ok(amount)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::{Local, NaiveDate};
    use crate::account::Account;
    use crate::input_processing::{add_new_transaction, add_transaction};

    #[test]
    fn add_today() {
        let account = Account::new("Savings");

        assert_eq!(account.transactions().len(), 0);
        assert_eq!(account.balance(), &0f32);

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec!(
            String::from("at"),
            String::from("Savings"),
            String::from("transaction1"),
            String::from("-10.00")
        );

        add_new_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(),1);

        let today = NaiveDate::from(Local::now().naive_local()).format("%d %b %Y");

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $-10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00",
                today
            )
        );

        assert_eq!(account_map.get("savings").unwrap().balance(), &-10f32);

        let inputs = vec!(
            String::from("at"),
            String::from("Savings"),
            String::from("transaction2"),
            String::from("30.00")
        );

        add_new_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("savings").unwrap().transactions().len(),2);
        assert_eq!(account_map.get("savings").unwrap().balance(), &20f32);

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $20.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00\n\
                   Date: {} | Label: Transaction 2 | Amount: $30.00",
                today, today
            )
        );
    }

    #[test]
    #[should_panic]
    fn add_today_short_input() {
        let account = Account::new("Savings");

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec!(
            String::from("at"),
            String::from("Savings"),
            String::from("transaction1"),
        );

        add_new_transaction(inputs, &mut account_map).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_today_wrong_name() {
        let account = Account::new("Savings");

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec!(
            String::from("at"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("-10.00")
        );

         add_new_transaction(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn add_specific_day() {
        let account = Account::new("Savings");

        assert_eq!(account.transactions().len(), 0);
        assert_eq!(account.balance(), &0f32);

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec!(
            String::from("at"),
            String::from("Savings"),
            String::from("transaction1"),
            String::from("-10.00"),
            String::from("2024-05-26"),
        );

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(),1);

        let today = "26 May 2024";

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $-10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00",
                today
            )
        );

        assert_eq!(account_map.get("savings").unwrap().balance(), &-10f32);
    }
}