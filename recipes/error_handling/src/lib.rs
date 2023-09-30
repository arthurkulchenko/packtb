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

pub fn get_sender_transaction(transactions: Vec<Transaction>, sender_name: &str) -> Result<Transaction, failure::Error> {
    // let transactions = get_transactions(file_name)?;
    for transaction in transactions {
        if transaction.from == sender_name {
            return Ok(transaction);
        }
    }
    Err(TransactionError::Message("No transactions").into())
    // let user_transactions = transactions.into_iter().filter(|transaction| transaction.from == sender_name).collect::<Vec<Transaction>>();
    // if user_transactions.len() == 0 { return None }

    // Some(user_transactions[0].clone())
}

pub fn get_transactions(file_name: &str) -> Result<Vec<Transaction>, failure::Error> {
    // std::fs::read_to_string(fname).map_err(|e| TransactionError::from(e))
    //     .and_then(|file| serde_json::from_str(&file).map_err(|e| TransactionError::from(e)))

    // Ok(
    //     match serde_json::from_str(
    //         &match std::fs::read_to_string(fname) {
    //             Ok(v) => v,
    //             Err(e) => return Err(e.into()),
    //         }
    //     ) {
    //         Ok(v) => v,
    //         Err(e) => return Err(e.into()),
    //     }
    // )
    Ok(serde_json::from_str(&std::fs::read_to_string(file_name)?)?) // ? is shorthand for the above

}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    pub fn getting_transactions() {
        assert_eq!(get_transactions("test_data/transactions.json").unwrap().len(), 4);
    }
}
