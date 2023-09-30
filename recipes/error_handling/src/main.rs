extern crate error_handling;
use error_handling::*;

fn main() -> Result<(), TransactionError>{
    let transactions = get_transactions("test_data/transactions.json")?;
    println!("{:?}", transactions);
    let first_transaction = get_sender_transaction("test_data/transactions.json", "Someone3").ok_or("No transactions")?;
    println!("{:?}", first_transaction);
    Ok(())
}
