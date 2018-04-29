use super::Simple;
use fmt::Render;
use stellar_client::resources::Transaction;

impl Render<Transaction> for Simple {
    fn render(&self, txn: &Transaction) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Hash:                    {}", txn.hash());
        append!(buf, "ledger:                  {}", txn.ledger());
        append!(buf, "created at:              {}", txn.created_at());
        append!(buf, "source account:          {}", txn.source_account());
        append!(
            buf,
            "source account sequence: {}",
            txn.source_account_sequence()
        );
        append!(buf, "fee paid:                {}", txn.fee_as_amount());
        append!(buf, "operation count:         {}", txn.operation_count());

        Some(buf)
    }
}
