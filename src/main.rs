mod account;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match budgeting_app::parse_file_path(&args) {
        Ok(file_path) => file_path,
        Err(e) => {
            println!("{}",e);
            exit(-1);
        }
    };

    if let Err(e) = budgeting_app::run(file_path) {
        println!("Application error: {e}");
        exit(-1);
    }
}