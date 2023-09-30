use serde::{Deserialize, Serialize};
use serde_json;
mod error;

pub use error::TransactionError;


#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64
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

pub fn get_first_transaction(fname: &str, uname: &str) -> Option<Transaction> {
    let transactions = get_transactions(fname).ok()?;
    for transaction in transactions {
        if transaction.from == uname {
            return Some(transaction);
        }
    }
    None
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // std::fs::read_to_string(fname).map_err(|e| TransactionError::from(e))
    //     .and_then(|file| serde_json::from_str(&file).map_err(|e| TransactionError::from(e)))
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?) // ? is shorthand for the above

}
