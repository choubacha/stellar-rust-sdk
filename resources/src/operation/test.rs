use serde_json;
use operation::{Operation, OperationKind};
use amount::Amount;
use asset::Flag;

fn account_merge_json() -> &'static str {
    include_str!("../../fixtures/operations/account_merge.json")
}

mod errors_on_missing_fields_for_type {
    use super::*;

    macro_rules! assert_err_on_missing_fields {
        ($type_to_check:ident, $type_i:expr) => {
            #[test]
            fn $type_to_check() {
                let json = format!(
                    r#"{{"id":"1","paging_token":"7","type_i":{},"type":"{}"}}"#,
                    $type_i,
                    stringify!($type_to_check),
                );
                let result = serde_json::from_str::<Operation>(&json);

                assert!(result.is_err());
                assert_eq!(
                    format!("{}", result.unwrap_err()),
                    format!("Missing fields for {} operation.", stringify!($type_to_check))
                );
            }
        }
    }

    assert_err_on_missing_fields!(create_account, 0);
    assert_err_on_missing_fields!(payment, 1);
    assert_err_on_missing_fields!(path_payment, 2);
    assert_err_on_missing_fields!(manage_offer, 3);
    assert_err_on_missing_fields!(create_passive_offer, 4);
    assert_err_on_missing_fields!(set_options, 5);
    assert_err_on_missing_fields!(change_trust, 6);
    assert_err_on_missing_fields!(allow_trust, 7);
    assert_err_on_missing_fields!(account_merge, 8);
    // Inflation (id 9) is infallible as it has no fields.
    assert_err_on_missing_fields!(manage_data, 10);
}

#[test]
fn it_parses_account_merge_from_json() {
    let operation: Operation = serde_json::from_str(&account_merge_json()).unwrap();
    assert!(operation.is_account_merge());
    assert_eq!(operation.type_i(), 8);
    if let &OperationKind::AccountMerge(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.account(),
            "GBCR5OVQ54S2EKHLBZMK6VYMTXZHXN3T45Y6PRX4PX4FXDMJJGY4FD42"
        );
        assert_eq!(
            account_details.into(),
            "GBS43BF24ENNS3KPACUZVKK2VYPOZVBQO2CISGZ777RYGOPYC2FT6S3K"
        );
    } else {
        panic!("Did not generate account merge kind");
    }
}

fn allow_trust_json() -> &'static str {
    include_str!("../../fixtures/operations/allow_trust.json")
}

#[test]
fn it_parses_allow_trust_from_json() {
    let operation: Operation = serde_json::from_str(&allow_trust_json()).unwrap();
    assert!(operation.is_allow_trust());
    assert_eq!(operation.type_i(), 7);
    if let &OperationKind::AllowTrust(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.trustee(),
            "GC23QF2HUE52AMXUFUH3AYJAXXGXXV2VHXYYR6EYXETPKDXZSAW67XO4"
        );
        assert_eq!(
            account_details.trustor(),
            "GBXGQJWVLWOYHFLVTKWV5FGHA3LNYY2JQKM7OAJAUEQFU6LPCSEFVXON"
        );
        assert_eq!(account_details.asset().code(), "USD");
        assert_eq!(account_details.authorize(), true);
    } else {
        panic!("Did not generate allow trust kind");
    }
}

fn change_trust_json() -> &'static str {
    include_str!("../../fixtures/operations/change_trust.json")
}

#[test]
fn it_parses_change_trust_from_json() {
    let operation: Operation = serde_json::from_str(&change_trust_json()).unwrap();
    assert!(operation.is_change_trust());
    assert_eq!(operation.type_i(), 6);
    if let &OperationKind::ChangeTrust(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.trustee(),
            "GAC2ZUXVI5266NMMGDPBMXHH4BTZKJ7MMTGXRZGX2R5YLMFRYLJ7U5EA"
        );
        assert_eq!(
            account_details.trustor(),
            "GDVXG2FMFFSUMMMBIUEMWPZAIU2FNCH7QNGJMWRXRD6K5FZK5KJS4DDR"
        );
        assert_eq!(account_details.asset().code(), "CHP");
        assert_eq!(account_details.limit(), Amount::new(50_000_000));
    } else {
        panic!("Did not generate change trust kind");
    }
}

fn create_account_json() -> &'static str {
    include_str!("../../fixtures/operations/create_account.json")
}

#[test]
fn it_parses_create_account_from_json() {
    let operation: Operation = serde_json::from_str(&create_account_json()).unwrap();
    assert!(operation.is_create_account());
    assert_eq!(operation.type_i(), 0);
    if let &OperationKind::CreateAccount(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.account(),
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
        );
        assert_eq!(
            account_details.funder(),
            "GBIA4FH6TV64KSPDAJCNUQSM7PFL4ILGUVJDPCLUOPJ7ONMKBBVUQHRO"
        );
        assert_eq!(
            account_details.starting_balance(),
            Amount::new(100_000_000_000)
        );
    } else {
        panic!("Did not generate create account kind");
    }
}

fn create_passive_offer_json() -> &'static str {
    include_str!("../../fixtures/operations/create_passive_offer.json")
}

#[test]
fn it_parses_a_create_passive_offer_from_json() {
    let operation: Operation = serde_json::from_str(&create_passive_offer_json()).unwrap();
    assert!(operation.is_create_passive_offer());
    assert_eq!(operation.type_i(), 4);
    if let &OperationKind::CreatePassiveOffer(ref account_details) = operation.kind() {
        assert_eq!(account_details.offer_id(), 9);
        assert_eq!(account_details.selling().code(), "XLM");
        assert_eq!(account_details.buying().code(), "USD");
        assert_eq!(account_details.amount(), Amount::new(112_782_700));
        assert_eq!(account_details.price_ratio().numerator(), 1);
        assert_eq!(account_details.price(), Amount::new(10_000_000));
    } else {
        panic!("Did not generate createpassive offer kind");
    }
}

fn inflation_json() -> &'static str {
    include_str!("../../fixtures/operations/inflation.json")
}

#[test]
fn it_parses_inflation_from_json() {
    let operation: Operation = serde_json::from_str(&inflation_json()).unwrap();
    assert!(operation.is_inflation());
    assert_eq!(operation.type_i(), 9);
}

fn manage_data_json() -> &'static str {
    include_str!("../../fixtures/operations/manage_data.json")
}

#[test]
fn it_parses_manage_data_from_json() {
    let operation: Operation = serde_json::from_str(&manage_data_json()).unwrap();
    assert!(operation.is_manage_data());
    assert_eq!(operation.type_i(), 10);
    if let &OperationKind::ManageData(ref account_details) = operation.kind() {
        assert_eq!(account_details.name(), "lang");
        assert_eq!(account_details.value(), "aW5kb25lc2lhbg==");
    } else {
        panic!("Did not generate manage data kind");
    }
}

fn manage_offer_json() -> &'static str {
    include_str!("../../fixtures/operations/manage_offer.json")
}

#[test]
fn it_parses_a_manage_offer_from_json() {
    let operation: Operation = serde_json::from_str(&manage_offer_json()).unwrap();
    assert!(operation.is_manage_offer());
    assert_eq!(operation.type_i(), 3);
    if let &OperationKind::ManageOffer(ref account_details) = operation.kind() {
        assert_eq!(account_details.offer_id(), 8);
        assert_eq!(account_details.selling().code(), "YEN");
        assert_eq!(account_details.buying().code(), "CHP");
        assert_eq!(account_details.amount(), Amount::new(1_000_000_000));
        assert_eq!(account_details.price_ratio().numerator(), 2);
        assert_eq!(account_details.price(), Amount::new(20_000_000));
    } else {
        panic!("Did not generate manage offer kind");
    }
}

fn path_payment_json() -> &'static str {
    include_str!("../../fixtures/operations/path_payment.json")
}

#[test]
fn it_parses_a_path_payment_from_json() {
    let operation: Operation = serde_json::from_str(&path_payment_json()).unwrap();
    assert!(operation.is_path_payment());
    assert_eq!(operation.type_i(), 2);
    if let &OperationKind::PathPayment(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.from(),
            "GCXKG6RN4ONIEPCMNFB732A436Z5PNDSRLGWK7GBLCMQLIFO4S7EYWVU"
        );
        assert_eq!(
            account_details.to(),
            "GA5WBPYA5Y4WAEHXWR2UKO2UO4BUGHUQ74EUPKON2QHV4WRHOIRNKKH2"
        );
        assert_eq!(account_details.destination_asset().code(), "EUR");
        assert_eq!(
            account_details.destination_amount(),
            Amount::new(100_000_000)
        );
        assert_eq!(account_details.source_asset().code(), "USD");
        assert_eq!(account_details.source_amount(), Amount::new(100_000_000));
        assert_eq!(account_details.source_max(), Amount::new(100_000_000));
    } else {
        panic!("Did not generate path payment kind");
    }
}

fn payment_json() -> &'static str {
    include_str!("../../fixtures/operations/payment.json")
}

#[test]
fn it_parses_a_payment_from_json() {
    let operation: Operation = serde_json::from_str(&payment_json()).unwrap();
    assert!(operation.is_payment());
    assert_eq!(operation.type_i(), 1);
    if let &OperationKind::Payment(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.from(),
            "GAKLBGHNHFQ3BMUYG5KU4BEWO6EYQHZHAXEWC33W34PH2RBHZDSQBD75"
        );
        assert_eq!(
            account_details.to(),
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
        );
        assert_eq!(account_details.asset().code(), "XLM");
        assert_eq!(account_details.amount(), Amount::new(2_000_000_000));
    } else {
        panic!("Did not generate payment kind");
    }
}

fn set_options_json() -> &'static str {
    include_str!("../../fixtures/operations/set_options.json")
}

#[test]
fn it_parses_a_set_options_from_json() {
    let operation: Operation = serde_json::from_str(&set_options_json()).unwrap();
    assert!(operation.is_set_options());
    assert_eq!(operation.type_i(), 5);
    if let &OperationKind::SetOptions(ref account_details) = operation.kind() {
        assert_eq!(
            account_details.signer_key(),
            "GA5WBPYA5Y4WAEHXWR2UKO2UO4BUGHUQ74EUPKON2QHV4WRHOIRNKKH2"
        );
        assert_eq!(account_details.signer_weight(), 1);
        assert_eq!(account_details.master_key_weight(), 2);
        assert_eq!(account_details.low_threshold(), 0);
        assert_eq!(account_details.med_threshold(), 3);
        assert_eq!(account_details.high_threshold(), 3);
        assert_eq!(account_details.home_domain(), "stellar.org");
        assert!(account_details.clear_flags().is_none());
        assert_eq!(account_details.set_flags().unwrap(), Flag::new(true, false));
    } else {
        panic!("Did not generate set options kind");
    }
}
