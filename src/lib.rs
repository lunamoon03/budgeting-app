use input_processing::*;
use itertools::Itertools;
use std::error::Error;
use std::io;

mod account;
mod file_processing;
mod input_processing;

fn print_menu() {
    println!("\n\
        ----------------------------------------------------------------------------------------------------\n\
        \tat  [account] [label] [amount] - add transaction from today to account\n\
        \tatd [account] [label] [amount] [date (YYYY-MM-DD)] - add transaction from another day to account\n\
        \taa  [account] - add new account\n\
        \tra  [account] - remove an account\n\
        \teta [account] [label] [amount] [date (YYYY-MM-DD)] [new amount] - edit amount of transaction\n\
        \tetd [account] [label] [amount] [date (YYYY-MM-DD)] [new date] - edit date of transaction\n\
        \tetl [account] [label] [amount] [date (YYYY-MM-DD)] [new label] - edit label of transaction\n\
        \trt  [account] [label] [amount] [date (YYYY-MM-DD)] - remove a transaction\n\
        \th   - show menu\n\
        \ts   - save changes\n\
        \tu   - undo all changes since last save\n\
        \tq   - exit program\n\
        ----------------------------------------------------------------------------------------------------\n");
}

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut accounts =
        file_processing::read_from_string(file_processing::get_file_contents(file_path)?)?;

    for account in accounts
        .values()
        .sorted_by(|a, b| Ord::cmp(a.name(), b.name()))
    {
        println!("{account}\n");
    }

    print_menu();

    loop {
        let mut trimmed_input: String;
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            trimmed_input = String::from(user_input.trim_end_matches('\n'));
            if !trimmed_input.is_empty() {
                break;
            }
        }

        let split_input = trimmed_input
            .split(' ')
            .map(String::from)
            .collect::<Vec<String>>();

        let mut result = Ok(());

        match split_input.first().unwrap_or(&"".to_string()).as_str() {
            "at" => result = add_new_transaction(split_input, &mut accounts),
            "atd" => result = add_transaction(split_input, &mut accounts),
            "aa" => result = add_account(split_input, &mut accounts),
            "ra" => result = remove_account(split_input, &mut accounts),
            "eta" => result = edit_transaction_amount(split_input, &mut accounts),
            "etd" => result = edit_transaction_date(split_input, &mut accounts),
            "etl" => result = edit_transaction_label(split_input, &mut accounts),
            "rt" => result = remove_transaction(split_input, &mut accounts),
            "h" => print_menu(),
            "s" => result = file_processing::write_to_file(file_path, &accounts),
            "u" => {
                accounts = file_processing::read_from_string(file_processing::get_file_contents(
                    file_path,
                )?)?
            }
            "q" => break,
            _ => println!("Please enter a valid input"),
        }

        if let Err(e) = result {
            println!("Error: {}", e)
        } else {
            println!();
            for account in accounts
                .values()
                .sorted_by(|a, b| Ord::cmp(a.name(), b.name()))
            {
                println!("{account}\n");
            }
        }
    }

    file_processing::write_to_file(file_path, &accounts)?;
    Ok(())
}

pub fn parse_args(args: &[String]) -> Result<&str, Box<dyn Error>> {
    let file_type = ".csv";
    if args.len() != 2 {
        return Err(Box::from(format!(
            "Enter only one argument: [filename{}]",
            file_type
        )));
    };

    let file_path = match args.get(1) {
        Some(file_path) if file_path.ends_with(file_type) => file_path,
        _ => {
            return Err(Box::from(format!(
                "Enter a valid file name: [filename{}]",
                file_type
            )))
        }
    };

    Ok(file_path)
}
