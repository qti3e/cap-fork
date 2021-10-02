use ic_certified_map::{fork, HashTree};
use ic_certified_map::{fork_hash, AsHashTree};
use ic_history_common::canister_list::CanisterList;
use ic_history_common::canister_map::CanisterMap;
use ic_kit::ic;
use serde::Serialize;

// It's ok.
use ic_history_common::user_canisters::UserCanisters;
use ic_history_common::*;
use ic_kit::macros::*;

mod upgrade;

/// Merkle tree of the canister.
///
/// 0: Canister Map
/// 1: User canisters
/// 2: Index canisters list
///
///      ROOT
///     /   \
///   / \    2
///  0   1
#[derive(Serialize)]
pub struct Data {
    /// Map: TokenContractId -> RootBucketId
    root_buckets: CanisterMap,
    /// Map each user to RootBucketId
    user_canisters: UserCanisters,
    /// List of the index canisters.
    index_canisters: CanisterList,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            root_buckets: CanisterMap::default(),
            user_canisters: UserCanisters::default(),
            index_canisters: {
                let mut list = CanisterList::default();
                list.push(ic::id());
                list
            },
        }
    }
}

#[query]
fn get_token_contract_root_bucket(
    arg: GetTokenContractRootBucketArg,
) -> GetTokenContractRootBucketResponse {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                fork(
                    data.root_buckets.gen_witness(&arg.canister),
                    HashTree::Pruned(data.user_canisters.root_hash()),
                ),
                HashTree::Pruned(data.index_canisters.root_hash()),
            )
            .into(),
        ),
    };

    let canister = data.root_buckets.get(&arg.canister).cloned();

    GetTokenContractRootBucketResponse { canister, witness }
}

#[query]
fn get_user_root_buckets(arg: GetUserRootBucketsArg) -> GetUserRootBucketsResponse<'static> {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                fork(
                    HashTree::Pruned(data.root_buckets.root_hash()),
                    data.user_canisters.witness(&arg.user),
                ),
                HashTree::Pruned(data.index_canisters.root_hash()),
            )
            .into(),
        ),
    };

    let contracts = data.user_canisters.get(&arg.user);

    GetUserRootBucketsResponse { contracts, witness }
}

#[query]
fn get_index_canisters(arg: WithWitnessArg) -> GetIndexCanistersResponse {
    let data = ic::get::<Data>();

    let witness = match arg.witness {
        false => None,
        true => Some(
            fork(
                HashTree::Pruned(fork_hash(
                    &data.root_buckets.root_hash(),
                    &data.user_canisters.root_hash(),
                )),
                data.index_canisters.as_hash_tree(),
            )
            .into(),
        ),
    };

    let canisters = data.index_canisters.to_vec();

    GetIndexCanistersResponse { canisters, witness }
}

#[update]
fn install_bucket_code(arg: RootBucketId) {
    todo!()
}