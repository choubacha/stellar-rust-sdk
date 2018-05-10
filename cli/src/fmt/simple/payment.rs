use super::Simple;
use fmt::Render;
use stellar_client::resources::{Operation, OperationKind};

impl Render<Operation> for Simple {
    fn render(&self, operation: &Operation) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "ID:             {}", operation.id());
        match *operation.kind() {
            OperationKind::CreateAccount(ref create_account) => {
                append_to_buffer!(buf, "Operation Kind:   Create Account");
                append_to_buffer!(buf, "Account:          {}", create_account.account());
                append_to_buffer!(buf, "Funder:           {}", create_account.funder());
                append_to_buffer!(
                    buf,
                    "Starting Balance: {}",
                    create_account.starting_balance()
                );
            }
            OperationKind::Payment(ref payment) => {
                append_to_buffer!(buf, "Operation Kind: Payment");
                append_to_buffer!(buf, "To account:     {}", payment.to());
                append_to_buffer!(buf, "From account:   {}", payment.from());
                append_to_buffer!(buf, "Asset Type:     {}", payment.asset().asset_type());
                append_to_buffer!(buf, "Asset Code:     {}", payment.asset().code());
                append_to_buffer!(buf, "Asset Issuer:   {}", payment.asset().issuer());
                append_to_buffer!(buf, "Amount:         {}", payment.amount());
            }
            OperationKind::PathPayment(ref path_payment) => {
                append_to_buffer!(buf, "Operation Kind:           Path Payment");
                append_to_buffer!(buf, "To account:               {}", path_payment.to());
                append_to_buffer!(buf, "From account:             {}", path_payment.from());
                append_to_buffer!(
                    buf,
                    "Source Asset Type:        {}",
                    path_payment.source_asset().asset_type()
                );
                append_to_buffer!(
                    buf,
                    "Source Asset Code:        {}",
                    path_payment.source_asset().code()
                );
                append_to_buffer!(
                    buf,
                    "Source Asset Issuer:      {}",
                    path_payment.source_asset().issuer()
                );
                append_to_buffer!(
                    buf,
                    "Destination Asset Type:   {}",
                    path_payment.destination_asset().asset_type()
                );
                append_to_buffer!(
                    buf,
                    "Destination Asset Code:   {}",
                    path_payment.destination_asset().code()
                );
                append_to_buffer!(
                    buf,
                    "Destination Asset Issuer: {}",
                    path_payment.destination_asset().issuer()
                );
                append_to_buffer!(
                    buf,
                    "Destination Amount:       {}",
                    path_payment.destination_amount()
                );
            }
            _ => (),
        }
        Some(buf)
    }
}
