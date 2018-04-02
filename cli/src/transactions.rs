use stellar_client::{endpoint::{transaction, Order}, error::Result, sync::{self, Client}};
use super::utils;

pub fn all<'a>(client: Client) -> Result<()> {
    let endpoint = transaction::All::default().order(Order::Desc);
    let iter = sync::Iter::new(&client, endpoint);

    for (i, result) in iter.enumerate() {
        let txn = result?;
        println!("ID:             {}", txn.id());
        println!("source account: {}", txn.source_account());
        println!("created at:     {}", txn.created_at());
        println!("");

        if (i + 1) % 10 == 0 && !utils::next_page() {
            break;
        }
    }
    Ok(())
}
