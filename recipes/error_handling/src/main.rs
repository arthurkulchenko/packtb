// mod transaction;
extern crate error_handling;

fn main() -> Result<(), TransactionError>{
    let transactions = get_transactions("test_data/transactions.json")?;
    println!("{:?}", transactions);
    let first_transaction = get_first_transaction("test_data/transactions.json", "Someone3").ok_or("No transactions")?;
    println!("{:?}", first_transaction);
    Ok(())
}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    pub fn getting_transactions() {
        assert_eq!(get_transactions("test_data/transactions.json").unwrap().len(), 4);
    }
}
