use resources::{asset::Flags,
                effect::{account::Kind as AccountKind, data::Kind as DataKind,
                         signer::Kind as SignerKind, trade::Kind as TradeKind,
                         trustline::Kind as TrustlineKind, Effect, EffectKind},
                Amount};
use serde_json;

fn account_created_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_created.json")
}

#[test]
fn it_parses_account_created_from_json() {
    let effect: Effect = serde_json::from_str(&account_created_json()).unwrap();
    assert!(effect.is_account_created());
    assert_eq!(effect.type_i(), 0);
    if let &EffectKind::Account(AccountKind::Created(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GBS43BF24ENNS3KPACUZVKK2VYPOZVBQO2CISGZ777RYGOPYC2FT6S3K"
        );
        assert_eq!(
            effect_details.starting_balance(),
            Amount::new(100_000_000_000_000)
        );
    } else {
        panic!("Did not generate account created kind");
    }
}

fn account_removed_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_removed.json")
}
#[test]
fn it_parses_account_removed_from_json() {
    let effect: Effect = serde_json::from_str(&account_removed_json()).unwrap();
    assert!(effect.is_account_removed());
    assert_eq!(effect.type_i(), 1);
    if let &EffectKind::Account(AccountKind::Removed(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GCBQ6JRBPF3SXQBQ6SO5MRBE7WVV4UCHYOSHQGXSZNPZLFRYVYOWBZRQ"
        );
    } else {
        panic!("Did not generate account removed kind");
    }
}

fn account_credited_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_credited.json")
}
#[test]
fn it_parses_account_credited_from_json() {
    let effect: Effect = serde_json::from_str(&account_credited_json()).unwrap();
    assert!(effect.is_account_credited());
    assert_eq!(effect.type_i(), 2);
    if let &EffectKind::Account(AccountKind::Credited(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GDLGTRIBFH24364GPWPUS45GUFC2GU4ARPGWTXVCPLGTUHX3IOS3ON47"
        );
        assert_eq!(effect_details.asset().code(), "XLM");
        assert_eq!(effect_details.amount(), Amount::new(10_000_000_000));
    } else {
        panic!("Did not generate account credited kind");
    }
}

fn account_debited_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_debited.json")
}
#[test]
fn it_parses_account_debited_from_json() {
    let effect: Effect = serde_json::from_str(&account_debited_json()).unwrap();
    assert!(effect.is_account_debited());
    assert_eq!(effect.type_i(), 3);
    if let &EffectKind::Account(AccountKind::Debited(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H"
        );
        assert_eq!(effect_details.asset().code(), "XLM");
        assert_eq!(effect_details.amount(), Amount::new(300_000_000));
    } else {
        panic!("Did not generate account debited kind");
    }
}

fn account_threshold_updated_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_threshold_updated.json")
}
#[test]
fn it_parses_account_thresholds_updated_from_json() {
    let effect: Effect = serde_json::from_str(&account_threshold_updated_json()).unwrap();
    assert!(effect.is_account_thresholds_updated());
    assert_eq!(effect.type_i(), 4);
    if let &EffectKind::Account(AccountKind::ThresholdsUpdated(ref effect_details)) = effect.kind()
    {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.low(), 2);
        assert_eq!(effect_details.med(), 3);
        assert_eq!(effect_details.high(), 4);
    } else {
        panic!("Did not generate account thresholds updated kind");
    }
}

fn account_home_domain_updated_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_home_domain_updated.json")
}
#[test]
fn it_parses_account_home_domain_updated_from_json() {
    let effect: Effect = serde_json::from_str(&account_home_domain_updated_json()).unwrap();
    assert!(effect.is_account_home_domain_updated());
    assert_eq!(effect.type_i(), 5);
    if let &EffectKind::Account(AccountKind::HomeDomainUpdated(ref effect_details)) = effect.kind()
    {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKO5"
        );
        assert_eq!(effect_details.home_domain(), "stellar.org");
    } else {
        panic!("Did not generate account home domain updated kind");
    }
}

fn account_flags_updated_json() -> &'static str {
    include_str!("../../../fixtures/effects/account_flags_updated.json")
}
#[test]
fn it_parses_account_flags_updated_from_json() {
    let effect: Effect = serde_json::from_str(&account_flags_updated_json()).unwrap();
    assert!(effect.is_account_flags_updated());
    assert_eq!(effect.type_i(), 6);
    if let &EffectKind::Account(AccountKind::FlagsUpdated(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.flags(), Flags::new(false, true));
    } else {
        panic!("Did not generate account flags updated kind");
    }
}

mod data {
    use super::*;

    fn data_updated_json() -> &'static str {
        include_str!("../../../fixtures/effects/data_updated.json")
    }
    #[test]
    fn it_parses_data_updated_from_json() {
        let effect: Effect = serde_json::from_str(&data_updated_json()).unwrap();
        assert!(effect.is_data_updated());
        assert_eq!(effect.type_i(), 42);
        if let &EffectKind::Data(DataKind::Updated(ref effect_details)) = effect.kind() {
            assert_eq!(
                effect_details.account(),
                "GDWGJSTUVRNFTR7STPUUHFWQYAN6KBVWCZT2YN7MY276GCSSXSWPS6JY"
            );
        } else {
            panic!("Did not generate account flags removed kind: {:?}", effect);
        }
    }

    fn data_created_json() -> &'static str {
        include_str!("../../../fixtures/effects/data_created.json")
    }
    #[test]
    fn it_parses_data_created_from_json() {
        let effect: Effect = serde_json::from_str(&data_created_json()).unwrap();
        assert!(effect.is_data_created());
        assert_eq!(effect.type_i(), 40);
        if let &EffectKind::Data(DataKind::Created(ref effect_details)) = effect.kind() {
            assert_eq!(
                effect_details.account(),
                "GDWGJSTUVRNFTR7STPUUHFWQYAN6KBVWCZT2YN7MY276GCSSXSWPS6JY"
            );
        } else {
            panic!("Did not generate account flags created kind: {:?}", effect);
        }
    }

    fn data_removed_json() -> &'static str {
        include_str!("../../../fixtures/effects/data_removed.json")
    }
    #[test]
    fn it_parses_data_removed_from_json() {
        let effect: Effect = serde_json::from_str(&data_removed_json()).unwrap();
        assert!(effect.is_data_removed());
        assert_eq!(effect.type_i(), 41);
        if let &EffectKind::Data(DataKind::Removed(ref effect_details)) = effect.kind() {
            assert_eq!(
                effect_details.account(),
                "GDWGJSTUVRNFTR7STPUUHFWQYAN6KBVWCZT2YN7MY276GCSSXSWPS6JY"
            );
        } else {
            panic!("Did not generate account flags removed kind: {:?}", effect);
        }
    }
}

fn signer_created_json() -> &'static str {
    include_str!("../../../fixtures/effects/signer_created.json")
}
#[test]
fn it_parses_signer_created_from_json() {
    let effect: Effect = serde_json::from_str(&signer_created_json()).unwrap();
    assert!(effect.is_signer_created());
    assert_eq!(effect.type_i(), 10);
    if let &EffectKind::Signer(SignerKind::Created(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GB24LPGAHYTWRYOXIDKXLI55SBRWW42T3TZKDAAW3BOJX4ADVIATFTLU"
        );
        assert_eq!(
            effect_details.public_key(),
            "GB24LPGAHYTWRYOXIDKXLI55SBRWW42T3TZKDAAW3BOJX4ADVIATFTLU"
        );
        assert_eq!(effect_details.weight(), 1);
    } else {
        panic!("Did not generate signer created kind");
    }
}

fn signer_removed_json() -> &'static str {
    include_str!("../../../fixtures/effects/signer_removed.json")
}
#[test]
fn it_parses_signer_removed_from_json() {
    let effect: Effect = serde_json::from_str(&signer_removed_json()).unwrap();
    assert!(effect.is_signer_removed());
    assert_eq!(effect.type_i(), 11);
    if let &EffectKind::Signer(SignerKind::Removed(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GCFKT6BN2FEASCEVDNHEC4LLFT2KLUUPEMKM4OJPEJ65H2AEZ7IH4RV6"
        );
        assert_eq!(
            effect_details.public_key(),
            "GCFKT6BN2FEASCEVDNHEC4LLFT2KLUUPEMKM4OJPEJ65H2AEZ7IH4RV6"
        );
        assert_eq!(effect_details.weight(), 0);
    } else {
        panic!("Did not generate signer removed kind");
    }
}

fn signer_updated_json() -> &'static str {
    include_str!("../../../fixtures/effects/signer_updated.json")
}
#[test]
fn it_parses_signer_updated_from_json() {
    let effect: Effect = serde_json::from_str(&signer_updated_json()).unwrap();
    assert!(effect.is_signer_updated());
    assert_eq!(effect.type_i(), 12);
    if let &EffectKind::Signer(SignerKind::Updated(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(
            effect_details.public_key(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.weight(), 2);
    } else {
        panic!("Did not generate signer updated kind");
    }
}

fn trustline_created_json() -> &'static str {
    include_str!("../../../fixtures/effects/trustline_created.json")
}
#[test]
fn it_parses_trustline_created_from_json() {
    let effect: Effect = serde_json::from_str(&trustline_created_json()).unwrap();
    assert!(effect.is_trustline_created());
    assert_eq!(effect.type_i(), 20);
    if let &EffectKind::Trustline(TrustlineKind::Created(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.asset().code(), "EUR");
        assert_eq!(effect_details.limit(), Amount::new(10_000_000_000));
    } else {
        panic!("Did not generate trustline created kind");
    }
}

fn trustline_removed_json() -> &'static str {
    include_str!("../../../fixtures/effects/trustline_removed.json")
}
#[test]
fn it_parses_trustline_removed_from_json() {
    let effect: Effect = serde_json::from_str(&trustline_removed_json()).unwrap();
    assert!(effect.is_trustline_removed());
    assert_eq!(effect.type_i(), 21);
    if let &EffectKind::Trustline(TrustlineKind::Removed(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "QA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.asset().code(), "EUR");
        assert_eq!(effect_details.limit(), Amount::new(0));
    } else {
        panic!("Did not generate trustline removed kind");
    }
}

fn trustline_updated_json() -> &'static str {
    include_str!("../../../fixtures/effects/trustline_updated.json")
}
#[test]
fn it_parses_trustline_updated_from_json() {
    let effect: Effect = serde_json::from_str(&trustline_updated_json()).unwrap();
    assert!(effect.is_trustline_updated());
    assert_eq!(effect.type_i(), 22);
    if let &EffectKind::Trustline(TrustlineKind::Updated(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.asset().code(), "TESTTEST");
        assert_eq!(effect_details.limit(), Amount::new(1_000_000_000));
    } else {
        panic!("Did not generate trustline updated kind");
    }
}

fn trustline_authorized_json() -> &'static str {
    include_str!("../../../fixtures/effects/trustline_authorized.json")
}
#[test]
fn it_parses_trustline_authorized_from_json() {
    let effect: Effect = serde_json::from_str(&trustline_authorized_json()).unwrap();
    assert!(effect.is_trustline_authorized());
    assert_eq!(effect.type_i(), 23);
    if let &EffectKind::Trustline(TrustlineKind::Authorized(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.asset().code(), "TESTTEST");
    } else {
        panic!("Did not generate trustline authorized kind");
    }
}

fn trustline_deauthorized_json() -> &'static str {
    include_str!("../../../fixtures/effects/trustline_deauthorized.json")
}
#[test]
fn it_parses_trustline_deauthorized_from_json() {
    let effect: Effect = serde_json::from_str(&trustline_deauthorized_json()).unwrap();
    assert!(effect.is_trustline_deauthorized());
    assert_eq!(effect.type_i(), 24);
    if let &EffectKind::Trustline(TrustlineKind::Deauthorized(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(effect_details.asset().code(), "EUR");
    } else {
        panic!("Did not generate trustline authorized kind");
    }
}

fn trade_json() -> &'static str {
    include_str!("../../../fixtures/effects/trade.json")
}
#[test]
fn it_parses_trade_from_json() {
    let effect: Effect = serde_json::from_str(&trade_json()).unwrap();
    assert!(effect.is_trade());
    assert_eq!(effect.type_i(), 33);
    if let &EffectKind::Trade(TradeKind::Trade(ref effect_details)) = effect.kind() {
        assert_eq!(
            effect_details.account(),
            "GA6U5X6WOPNKKDKQULBR7IDHDBAQKOWPHYEC7WSXHZBFEYFD3XVZAKOO"
        );
        assert_eq!(
            effect_details.seller(),
            "GCVHDLN6EHZBYW2M3BQIY32C23E4GPIRZZDBNF2Q73DAZ5VJDRGSMYRB"
        );
        assert_eq!(effect_details.offer_id(), 1);
        assert_eq!(effect_details.sold_asset().code(), "EUR");
        assert_eq!(effect_details.sold_amount(), Amount::new(10_000_000_000));
        assert_eq!(effect_details.bought_asset().code(), "TESTTEST");
        assert_eq!(effect_details.bought_amount(), Amount::new(600_000_000));
    } else {
        panic!("Did not generate trustline authorized kind");
    }
}

mod errors_on_missing_fields_for_effect_types {
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
                let result = serde_json::from_str::<Effect>(&json);

                assert!(result.is_err());
                assert_eq!(
                    format!("{}", result.unwrap_err()),
                    format!("Missing fields for {} effect.", stringify!($type_to_check))
                );
            }
        };
    }

    assert_err_on_missing_fields!(account_created, 0);
    assert_err_on_missing_fields!(account_removed, 1);
    assert_err_on_missing_fields!(account_credited, 2);
    assert_err_on_missing_fields!(account_debited, 3);
    assert_err_on_missing_fields!(account_thresholds_updated, 4);
    assert_err_on_missing_fields!(account_home_domain_updated, 5);
    assert_err_on_missing_fields!(account_flags_updated, 6);
    assert_err_on_missing_fields!(signer_created, 10);
    assert_err_on_missing_fields!(signer_removed, 11);
    assert_err_on_missing_fields!(trustline_created, 20);
    assert_err_on_missing_fields!(trustline_removed, 21);
    assert_err_on_missing_fields!(trustline_updated, 22);
    assert_err_on_missing_fields!(trustline_authorized, 23);
    assert_err_on_missing_fields!(trade, 33);
    assert_err_on_missing_fields!(data_created, 40);
    assert_err_on_missing_fields!(data_removed, 41);
    assert_err_on_missing_fields!(data_updated, 42);
}
