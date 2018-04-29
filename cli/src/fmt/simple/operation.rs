use super::Simple;
use fmt::Render;
use stellar_client::resources::{OperationKind as Kind, operation::*};

impl Render<Operation> for Simple {
    fn render(&self, op: &Operation) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "ID:   {}", op.id());
        append!(buf, "Kind: {}", op.kind_name());
        let kind_details = match op.kind() {
            Kind::CreateAccount(kind) => self.render(kind),
            Kind::Payment(kind) => self.render(kind),
            Kind::PathPayment(kind) => self.render(kind),
            Kind::ManageOffer(kind) => self.render(kind),
            Kind::CreatePassiveOffer(kind) => self.render(kind),
            Kind::SetOptions(kind) => self.render(kind),
            Kind::ChangeTrust(kind) => self.render(kind),
            Kind::AllowTrust(kind) => self.render(kind),
            Kind::AccountMerge(kind) => self.render(kind),
            Kind::Inflation => None,
            Kind::ManageData(kind) => self.render(kind),
        };
        if let Some(value) = kind_details {
            append!(buf, "{}", value);
        }
        Some(buf)
    }
}

impl Render<CreateAccount> for Simple {
    fn render(&self, op: &CreateAccount) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Account:          {}", op.account());
        append!(buf, "Funder:           {}", op.funder());
        append!(buf, "Starting Balance: {}", op.starting_balance());
        Some(buf)
    }
}

impl Render<Payment> for Simple {
    fn render(&self, op: &Payment) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "To Account:   {}", op.to());
        append!(buf, "From Account: {}", op.from());
        append!(buf, "Asset:        {}", self.render(op.asset()).unwrap());
        append!(buf, "Amount:       {}", op.amount());
        Some(buf)
    }
}

impl Render<PathPayment> for Simple {
    fn render(&self, op: &PathPayment) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "To Account:         {}", op.to());
        append!(buf, "From Account:       {}", op.from());
        append!(
            buf,
            "Source Asset:       {}",
            self.render(op.source_asset()).unwrap()
        );
        append!(
            buf,
            "Destination Asset:  {}",
            self.render(op.destination_asset()).unwrap()
        );
        append!(buf, "Destination Amount: {}", op.destination_amount());
        Some(buf)
    }
}

impl Render<ManageOffer> for Simple {
    fn render(&self, op: &ManageOffer) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Offer ID:      {}", op.offer_id());
        append!(buf, "Selling Asset: {}", self.render(op.selling()).unwrap());
        append!(buf, "Buying Asset:  {}", self.render(op.buying()).unwrap());
        append!(buf, "Amount Sold:  {}", op.amount());
        append!(buf, "Price Ratio:  {}", op.price_ratio());
        append!(buf, "Price:        {}", op.price());
        Some(buf)
    }
}

impl Render<CreatePassiveOffer> for Simple {
    fn render(&self, op: &CreatePassiveOffer) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Offer ID:      {}", op.offer_id());
        append!(buf, "Selling Asset: {}", self.render(op.selling()).unwrap());
        append!(buf, "Buying Asset:  {}", self.render(op.buying()).unwrap());
        append!(buf, "Amount Sold:  {}", op.amount());
        append!(buf, "Price Ratio:  {}", op.price_ratio());
        append!(buf, "Price:        {}", op.price());
        Some(buf)
    }
}

impl Render<SetOptions> for Simple {
    fn render(&self, op: &SetOptions) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "New Signer Key:    {}", op.signer_key());
        append!(buf, "Signer Key Weight: {}", op.signer_weight());
        append!(buf, "Master Key Weight: {}", op.master_key_weight());
        append!(buf, "Thresholds:");
        indent!(buf, self, "Low:    {}", op.low_threshold());
        indent!(buf, self, "Medium: {}", op.med_threshold());
        indent!(buf, self, "High:   {}", op.high_threshold());
        append!(buf, "Set Flags:");
        if let Some(flags) = op.set_flags() {
            nest!(buf, self, &flags);
        } else {
            indent!(buf, self, "None");
        }
        append!(buf, "Cleared Flags:");
        if let Some(flags) = op.clear_flags() {
            nest!(buf, self, &flags);
        } else {
            indent!(buf, self, "None");
        }
        Some(buf)
    }
}

impl Render<ChangeTrust> for Simple {
    fn render(&self, op: &ChangeTrust) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Trustee:     {}", op.trustee());
        append!(buf, "Trustor:     {}", op.trustor());
        append!(buf, "Asset:       {}", self.render(op.asset()).unwrap());
        append!(buf, "Asset Limit: {}", op.limit());
        Some(buf)
    }
}

impl Render<AllowTrust> for Simple {
    fn render(&self, op: &AllowTrust) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Trustee:      {}", op.trustee());
        append!(buf, "Trustor:      {}", op.trustor());
        append!(buf, "Trust Status: {}", render_trust_status(op));
        append!(buf, "Asset:        {}", self.render(op.asset()).unwrap());
        Some(buf)
    }
}

fn render_trust_status(op: &AllowTrust) -> &str {
    if op.authorize() {
        "allowed"
    } else {
        "revoked"
    }
}

impl Render<AccountMerge> for Simple {
    fn render(&self, op: &AccountMerge) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Account Deleted:        {}", op.account());
        append!(buf, "Funds Transferred Into: {}", op.into());
        Some(buf)
    }
}

impl Render<ManageData> for Simple {
    fn render(&self, op: &ManageData) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "Name:  {}", op.name());
        append!(buf, "Value: {}", op.value());
        Some(buf)
    }
}
