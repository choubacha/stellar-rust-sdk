use super::Simple;
use fmt::Render;
use stellar_client::resources::Account;

impl Render<Account> for Simple {
    fn render(&self, account: &Account) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "ID:       {}", account.id());
        append_to_buffer!(buf, "Sequence: {}", account.sequence());
        Some(buf)
    }
}
