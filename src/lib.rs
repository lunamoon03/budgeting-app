use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let _contents = get_file_contents(file_path)?;

    Ok(())
}

pub fn parse_file_path(args: &[String]) -> Result<&str, Box<dyn Error>> {
    let file_type = ".csv";
    if args.len()!=2 {
        return Err(Box::from("Enter only one argument: [filename.txt]"));
    };

    let file_path = match args.get(1) {
        Some(file_path) if file_path.ends_with(file_type) => file_path,
        _ => return Err(Box::from("Enter a valid filename: [filename.txt]")),
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