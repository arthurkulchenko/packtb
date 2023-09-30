use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64
}

fn main() {
    let transactions = get_transactions("test_data/transactions.json").expect("Could not load transactions");
    println!("{:?}", transactions);
}

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    match std::fs::read_to_string(fname) {
        Ok(file) => {
            match serde_json::from_str::<Vec<Transaction>>(&file) {
                Ok(transactions) if (transactions.len() == 0) => Err("No transactions".to_string()),
                Ok(transactions) if (transactions.len() > 0) => Ok(transactions),
                _ => Err("Can't parse".to_string()),
            }
        },
        Err(..) => Err("No such file".to_string()),
    }
    
}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    pub fn getting_transactions() {
        assert_eq!(get_transactions("test_data/transactions.json").unwrap().len(), 4);
    }
}
