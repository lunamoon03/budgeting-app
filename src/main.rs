use crate::account::Account;

mod account;

fn main() {
    let mut a = Account::new("test");
    a.add_transaction("", 15f32).unwrap_or_else(|error| {
        println!("{}", error);
    });
}
