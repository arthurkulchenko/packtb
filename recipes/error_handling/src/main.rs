use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64
}

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    NoTransactions,
}

impl From<std::io::Error> for TransactionError {
    fn from(err: std::io::Error) -> TransactionError {
        TransactionError::LoadError(err)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(err: serde_json::Error) -> TransactionError {
        TransactionError::ParseError(err)
    }
}

fn main() {
    let transactions = get_transactions("test_data/transactions.json").expect("Could not load transactions");
    println!("{:?}", transactions);
}

// fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
//     match std::fs::read_to_string(fname) {
//         Ok(file) => {
//             match serde_json::from_str::<Vec<Transaction>>(&file) {
//                 Ok(transactions) if (transactions.len() == 0) => Err("No transactions".to_string()),
//                 Ok(transactions) if (transactions.len() > 0) => Ok(transactions),
//                 _ => Err("Can't parse".to_string()),
//             }
//         },
//         Err(..) => Err("No such file".to_string()),
//     }
    
// }

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // std::fs::read_to_string(fname).map_err(|e| TransactionError::from(e))
    //     .and_then(|file| serde_json::from_str(&file).map_err(|e| TransactionError::from(e)))
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?) // ? is shorthand for the above

}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    pub fn getting_transactions() {
        assert_eq!(get_transactions("test_data/transactions.json").unwrap().len(), 4);
    }
}
