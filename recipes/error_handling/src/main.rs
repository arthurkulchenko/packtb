extern crate error_handling;
use error_handling::*;
use failure::Error;

fn main() -> Result<(), Error>{
    let transactions = get_transactions("test_data/transactions.json")?;
    // println!("{:?}", transactions);
    let sender = fetch_sender_transaction(&transactions, "Someone1334").ok_or("No transactions");
    println!("{:?}", sender);

    match get_sender_transaction(transactions, "Someone3") {
        Ok(transaction) => println!("{:?}", transaction),
        Err(e) => println!("{}, Backtrace = : {}", e, e.backtrace()),
    }
    Ok(())
}
