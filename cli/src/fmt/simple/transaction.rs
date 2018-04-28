use fmt::Render;
use super::Simple;
use stellar_client::resources::Transaction;

impl Render<Transaction> for Simple {
    fn render(&self, txn: &Transaction) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "ID:             {}", txn.id());
        append_to_buffer!(buf, "source account: {}", txn.source_account());
        append_to_buffer!(buf, "created at:     {}", txn.created_at());

        Some(buf)
    }
}
