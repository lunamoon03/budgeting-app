use std::error::Error;
use std::io;

mod account;
mod file_processing;

fn print_menu() {
    println!("\n\
        ----------------------------------------------------------------------------------------------------\n\
        \tat  [account] [label] [amount] - add transaction from today to account\n\
        \tatd [account] [label] [amount] [date (YYYY-MM-DD)] - add transaction from another day to account\n\
        \taa  [account] - add new account\n\
        \teta [account] [label] [amount] - edit amount of transaction\n\
        \tetd [account] [label] [date (YYYY-MM-DD)] - edit date of transaction\n\
        \tetl [account] [label] [new label] - edit label of transaction\n\
        \trt  [account] [label] - remove a transaction\n\
        \th   - show menu\n\
        \tq   - exit program\n\
        ----------------------------------------------------------------------------------------------------\n");
}

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let accounts = file_processing::read_from_string(file_processing::get_file_contents(file_path)?)?;

    for account in accounts.values() {
        println!("{account}\n");
    }

    //let inputs: HashMap<String, fn(String) -> Result<(), Box<dyn Error>>> = HashMap::from(["at","atd","aa","eta","etd","etl","rt","q"]);
    //let inputs: HashSet<&str> = HashSet::from(["at", "atd", "aa", "eta", "etd", "etl", "rt", "q"]);
    print_menu();


    loop {
        let mut trimmed_input: String;
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            trimmed_input = String::from(user_input.trim_end_matches('\n'));
            if !trimmed_input.is_empty() { break; }
        }

        match *trimmed_input.split(" ").collect::<Vec<&str>>().first().unwrap_or(&""){
            "at" => println!("at"),
            "atd" => println!("atd"),
            "aa" => println!("aa"),
            "eta" => println!("eta"),
            "etd" => println!("etd"),
            "etl" => println!("etl"),
            "rt" => println!("rt"),
            "h" => print_menu(),
            "q" => break,
            _ => println!("Please enter a valid input"),
        }
    }

    file_processing::write_to_file(file_path, accounts)?;
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
