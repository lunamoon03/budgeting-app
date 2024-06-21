use crate::account::Account;
use chrono::NaiveDate;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;

pub(super) fn add_new_transaction(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 4)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.add_new_transaction(&label, amount)?;

    Ok(())
}

pub(super) fn add_transaction(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 5)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let date = get_transaction_date(&inputs)?;

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.add_transaction(&label, amount, date)?;

    Ok(())
}

pub(super) fn add_account(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 2)?;

    let account_name = inputs.get(1).unwrap().to_case(Case::Title);

    if accounts.keys().contains(&account_name.to_lowercase()) {
        return Err(Box::from(
            format!("Account {account_name} already exists.",),
        ));
    }

    let new_account = Account::new(&account_name);
    accounts.insert(account_name.to_lowercase(), new_account);

    Ok(())
}

pub(super) fn remove_account(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 2)?;

    let account_name = inputs.get(1).unwrap().to_case(Case::Title);

    if !accounts.keys().contains(&account_name.to_lowercase()) {
        return Err(Box::from(
            format!("Account {account_name} does not exist.",),
        ));
    }

    accounts.remove(&account_name.to_lowercase());

    Ok(())
}

pub(super) fn edit_transaction_amount(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 6)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let date = get_transaction_date(&inputs)?;

    let new_amount: f32 = match inputs.get(5).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Box::from(format!("Amount entered invalid: {}", e))),
    };

    let transaction_key = format!("{}-{}-{}", date, label, amount);

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.edit_transaction_amount(&transaction_key, new_amount)?;

    Ok(())
}

pub(super) fn edit_transaction_date(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 6)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let date = get_transaction_date(&inputs)?;

    let new_date: NaiveDate = match inputs.get(5).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Box::from(format!("Date entered invalid: {}", e))),
    };

    let transaction_key = format!("{}-{}-{}", date, label, amount);

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.edit_transaction_date(&transaction_key, new_date)?;

    Ok(())
}

pub(super) fn edit_transaction_label(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 6)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let date = get_transaction_date(&inputs)?;

    let new_label = inputs.get(5).unwrap().to_case(Case::Title);

    let transaction_key = format!("{}-{}-{}", date, label, amount);

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.edit_transaction_label(&transaction_key, new_label)?;

    Ok(())
}

pub(super) fn remove_transaction(
    inputs: Vec<String>,
    accounts: &mut HashMap<String, Account>,
) -> Result<(), Box<dyn Error>> {
    check_input_length(&inputs, 5)?;

    // Can unwrap freely due to check of input len
    let account_name = get_account_name(&inputs, accounts)?;

    let label = inputs.get(2).unwrap().to_case(Case::Title);

    let amount = get_transaction_amount(&inputs)?;

    let date = get_transaction_date(&inputs)?;

    let transaction_key = format!("{}-{}-{}", date, label, amount);

    let account: &mut Account = match accounts.get_mut(&account_name) {
        Some(a) => a,
        None => return Err(Box::from(format!("Account name {account_name} invalid"))),
    };

    account.remove_transaction(&transaction_key)?;

    Ok(())
}

fn check_input_length(inputs: &Vec<String>, input_length: usize) -> Result<(), Box<dyn Error>> {
    if inputs.len() != input_length {
        return Err(Box::from(format!(
            "Wrong number of inputs. {} when it should be {}",
            inputs.len() - 1,
            input_length - 1
        )));
    }
    Ok(())
}

fn get_account_name(
    inputs: &[String],
    accounts: &HashMap<String, Account>,
) -> Result<String, Box<dyn Error>> {
    let account_name = inputs.get(1).unwrap().to_lowercase();
    if !accounts.keys().contains(&account_name) {
        return Err(Box::from(format!(
            "Account name {} not present.",
            account_name.to_case(Case::Title)
        )));
    }
    Ok(account_name)
}

fn get_transaction_amount(inputs: &[String]) -> Result<f32, Box<dyn Error>> {
    let amount: f32 = match inputs.get(3).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Box::from(format!("Amount entered invalid: {}", e))),
    };
    Ok(amount)
}

fn get_transaction_date(inputs: &[String]) -> Result<NaiveDate, Box<dyn Error>> {
    let date: NaiveDate = match inputs.get(4).unwrap().parse() {
        Ok(f) => f,
        Err(e) => return Err(Box::from(format!("Date entered invalid: {}", e))),
    };
    Ok(date)
}

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use crate::input_processing::{
        add_account, add_new_transaction, add_transaction, edit_transaction_amount,
        edit_transaction_date, edit_transaction_label, remove_account, remove_transaction,
    };
    use chrono::format::{DelayedFormat, StrftimeItems};
    use chrono::{Local, NaiveDate};
    use std::collections::HashMap;

    fn today() -> NaiveDate {
        NaiveDate::from(Local::now().naive_local())
    }

    fn today_formatted<'a>() -> DelayedFormat<StrftimeItems<'a>> {
        NaiveDate::from(Local::now().naive_local()).format("%d %b %Y")
    }

    #[test]
    fn add_today() {
        let account = Account::new("Savings");

        assert_eq!(account.transactions().len(), 0);
        assert_eq!(account.balance(), &0f32);

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let past_date: NaiveDate = "2024-05-25".parse().unwrap();
        let inputs = vec![
            String::from("at"),
            String::from("Savings"),
            String::from("transaction1"),
            String::from("-10.00"),
            past_date.to_string(),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(), 1);

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $-10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00",
                past_date.format("%d %b %Y")
            )
        );

        assert_eq!(account_map.get("savings").unwrap().balance(), &-10f32);

        let inputs = vec![
            String::from("at"),
            String::from("Savings"),
            String::from("transaction2"),
            String::from("30.00"),
        ];

        add_new_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("savings").unwrap().transactions().len(), 2);
        assert_eq!(account_map.get("savings").unwrap().balance(), &20f32);

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $20.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00\n\
                   Date: {} | Label: Transaction 2 | Amount: $30.00",
                past_date.format("%d %b %Y"),
                today_formatted()
            )
        );
    }

    #[test]
    #[should_panic]
    fn add_today_short_input() {
        let account = Account::new("Savings");

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec![
            String::from("at"),
            String::from("Savings"),
            String::from("transaction1"),
        ];

        add_new_transaction(inputs, &mut account_map).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_today_wrong_name() {
        let account = Account::new("Savings");

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec![
            String::from("at"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("-10.00"),
        ];

        add_new_transaction(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn add_specific_day() {
        let account = Account::new("Savings");

        assert_eq!(account.transactions().len(), 0);
        assert_eq!(account.balance(), &0f32);

        let mut account_map = HashMap::new();
        account_map.insert(account.name().to_lowercase(), account);

        let inputs = vec![
            String::from("atd"),
            String::from("Savings"),
            String::from("transaction1"),
            String::from("-10.00"),
            String::from("2024-05-26"),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(), 1);

        let date = "26 May 2024";

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            &format!(
                "Name: Savings | Balance: $-10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $-10.00",
                date
            )
        );

        assert_eq!(account_map.get("savings").unwrap().balance(), &-10f32);
    }

    #[test]
    fn add_account_test() {
        let mut account_map = HashMap::new();
        let inputs = vec![String::from("aa"), String::from("Savings")];

        add_account(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(), 1);
        assert_eq!(account_map.get("savings").unwrap().balance(), &0f32);
        assert_eq!(account_map.get("savings").unwrap().transactions().len(), 0);

        assert_eq!(
            &format!("{}", account_map.get("savings").unwrap()),
            "Name: Savings | Balance: $0.00\n\
            Transactions:\n"
        );
    }

    #[test]
    #[should_panic]
    fn add_account_already_exists() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("savings"), Account::new("Savings"));

        let inputs = vec![String::from("aa"), String::from("Savings")];

        add_account(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn remove_account_test() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("savings"), Account::new("Savings"));

        let inputs = vec![String::from("aa"), String::from("Savings")];

        remove_account(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.len(), 0);
    }

    #[test]
    #[should_panic]
    fn remove_account_does_not_exist() {
        let mut account_map = HashMap::new();

        let inputs = vec![String::from("aa"), String::from("Savings")];

        remove_account(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn edit_transaction_amount_test() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &format!(
                "Name: Expenses | Balance: $10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $10.00",
                today_formatted()
            )
        );

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
            String::from("20.00"),
        ];

        edit_transaction_amount(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &format!(
                "Name: Expenses | Balance: $20.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $20.00",
                today_formatted()
            )
        );
    }

    #[test]
    #[should_panic]
    fn edit_transaction_amount_doesnt_exist() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        edit_transaction_amount(inputs, &mut account_map).unwrap();
    }

    #[test]
    #[should_panic]
    fn edit_transaction_amount_collision() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("20.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
            String::from("20.00"),
        ];

        edit_transaction_amount(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn edit_transaction_date_test() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-25"),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &"Name: Expenses | Balance: $10.00\n\
            Transactions:\n\
            Date: 25 May 2024 | Label: Transaction 1 | Amount: $10.00"
                .to_string()
        );

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-25"),
            String::from("2024-05-26"),
        ];

        edit_transaction_date(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &"Name: Expenses | Balance: $10.00\n\
            Transactions:\n\
            Date: 26 May 2024 | Label: Transaction 1 | Amount: $10.00"
                .to_string()
        );
    }

    #[test]
    #[should_panic]
    fn edit_transaction_date_doesnt_exist() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-25"),
            String::from("2024-05-26"),
        ];

        edit_transaction_date(inputs, &mut account_map).unwrap();
    }

    #[test]
    #[should_panic]
    fn edit_transaction_date_collision() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-25"),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-26"),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            String::from("2024-05-25"),
            String::from("2024-05-26"),
        ];

        edit_transaction_date(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn edit_transaction_label_test() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &format!(
                "Name: Expenses | Balance: $10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 1 | Amount: $10.00",
                today_formatted()
            )
        );

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
            String::from("transaction2"),
        ];

        edit_transaction_label(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);
        assert_eq!(
            &format!("{}", account_map.get("expenses").unwrap()),
            &format!(
                "Name: Expenses | Balance: $10.00\n\
                   Transactions:\n\
                   Date: {} | Label: Transaction 2 | Amount: $10.00",
                today_formatted()
            )
        );
    }

    #[test]
    #[should_panic]
    fn edit_transaction_label_doesnt_exist() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
            String::from("transaction2"),
        ];

        edit_transaction_label(inputs, &mut account_map).unwrap();
    }

    #[test]
    #[should_panic]
    fn edit_transaction_label_collision() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction2"),
            String::from("10.00"),
            format!("{}", today()),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction1"),
            String::from("10.00"),
            format!("{}", today()),
            String::from("transaction2"),
        ];

        edit_transaction_date(inputs, &mut account_map).unwrap();
    }

    #[test]
    fn remove_transaction_test() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let mut inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction2"),
            String::from("10.00"),
            String::from("2024-05-26"),
        ];

        add_transaction(inputs, &mut account_map).unwrap();

        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 1);

        inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction2"),
            String::from("10.00"),
            String::from("2024-05-26"),
        ];

        remove_transaction(inputs, &mut account_map).unwrap();
        assert_eq!(account_map.get("expenses").unwrap().transactions().len(), 0);
        assert_eq!(account_map.get("expenses").unwrap().balance(), &0f32);
    }

    #[test]
    #[should_panic]
    fn remove_transaction_does_not_exist() {
        let mut account_map = HashMap::new();
        account_map.insert(String::from("expenses"), Account::new("Expenses"));

        let inputs = vec![
            String::from("rt"),
            String::from("Expenses"),
            String::from("transaction2"),
            String::from("10.00"),
            String::from("2024-05-26"),
        ];

        remove_transaction(inputs, &mut account_map).unwrap();
    }
}
