use fmt::Render;
use super::Simple;
use stellar_client::resources::Transaction;

impl Render<Transaction> for Simple {
    fn render(&self, txn: &Transaction) -> Option<String> {
        let mut buf = String::new();

        macro_rules! append {
            ($($args:tt)*) => {
                buf.push_str(&format!($($args)*));
                buf.push_str("\n");
            }
        }
        append!("ID:             {}", txn.id());
        append!("source account: {}", txn.source_account());
        append!("created at:     {}", txn.created_at());

        Some(buf)
    }
}
