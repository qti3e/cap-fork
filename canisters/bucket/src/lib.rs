use cap_common::bucket::Bucket;
use cap_common::did::*;
use ic_kit::candid::candid_method;
use ic_kit::macros::*;
use ic_kit::{ic, Principal};
use serde::{Deserialize, Serialize};

mod upgrade;

#[derive(Serialize, Deserialize)]
struct Data {
    bucket: Bucket,
    parent: Principal,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            bucket: Bucket::new(Principal::management_canister(), 0),
            parent: Principal::management_canister(),
        }
    }
}

#[init]
fn init(arg: BucketInitArgs) {
    let mut data = ic::get_mut::<Data>();
    data.bucket = Bucket::new(arg.contract, arg.offset);
    data.bucket.set_next_canisters(arg.next_canisters);
    data.parent = ic::caller();
}

#[query]
#[candid_method(query)]
fn get_next_canisters(arg: WithWitnessArg) -> GetNextCanistersResponse {
    ic::get::<Data>().bucket.get_next_canisters(arg)
}

#[query]
#[candid_method(query)]
fn get_transaction(arg: WithIdArg) -> GetTransactionResponse {
    ic::get::<Data>().bucket.get_transaction(arg)
}

#[query]
#[candid_method(query)]
fn get_transactions(arg: GetTransactionsArg) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_user_transactions(arg: GetUserTransactionsArg) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_user_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_token_transactions(
    arg: GetTokenTransactionsArg,
) -> GetTransactionsResponseBorrowed<'static> {
    ic::get::<Data>().bucket.get_token_transactions(arg)
}

#[query]
#[candid_method(query)]
fn get_bucket_for(arg: WithIdArg) -> GetBucketResponse {
    ic::get::<Data>().bucket.get_bucket_for(arg)
}

#[query]
#[candid_method(query)]
fn size() -> u64 {
    ic::get::<Data>().bucket.size()
}

#[query]
#[candid_method(query)]
fn contract_id() -> &'static Principal {
    ic::get::<Data>().bucket.contract_id()
}

#[query]
#[candid_method(query)]
fn balance() -> u64 {
    ic::balance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("candid");
        write(dir.join("bucket.did"), export_candid()).expect("Write failed.");
    }
}
