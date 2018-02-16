use utils;

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    account_id: String,
    #[serde(deserialize_with = "utils::deserialize_from_str")]
    sequence: u64,
    subentry_count: u64,
}
