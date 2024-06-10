use std::error::Error;

mod account;
mod file_processing;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = file_processing::get_file_contents(file_path)?;

    let _accounts = file_processing::read_from_file(contents);

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