use crate::account::Account;

mod account;

fn main() {
    let mut a = Account::new("test");

    for i in 0..20 {
        a.add_transaction(&format!("Transaction {}", i), i as f32).unwrap_or_else(|error| {
            println!("{}", error);
        });
    }

    println!("{}",a);

}
