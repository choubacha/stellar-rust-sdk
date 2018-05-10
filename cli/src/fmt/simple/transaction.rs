use super::Simple;
use fmt::Render;
use stellar_client::resources::Transaction;

impl Render<Transaction> for Simple {
    fn render(&self, txn: &Transaction) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "Hash:                    {}", txn.hash());
        append_to_buffer!(buf, "ledger:                  {}", txn.ledger());
        append_to_buffer!(buf, "created at:              {}", txn.created_at());
        append_to_buffer!(buf, "source account:          {}", txn.source_account());
        append_to_buffer!(
            buf,
            "source account sequence: {}",
            txn.source_account_sequence()
        );
        append_to_buffer!(buf, "fee paid:                {}", txn.fee_as_amount());
        append_to_buffer!(buf, "operation count:         {}", txn.operation_count());

        Some(buf)
    }
}
