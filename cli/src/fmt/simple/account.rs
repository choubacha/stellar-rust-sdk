use fmt::Render;
use super::Simple;
use stellar_client::resources::Account;

impl Render<Account> for Simple {
    fn render(&self, account: &Account) -> Option<String> {
        let mut buf = String::new();
        buf.push_str(&format!("ID:       {}\n", account.id()));
        buf.push_str(&format!("Sequence: {}\n", account.sequence()));
        Some(buf)
    }
}
