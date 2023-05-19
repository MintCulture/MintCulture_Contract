use crate::*;
use near_sdk:: {CryptoHash};
use std::mem::size_of;
use near_sdk::Promise;

// 스토리지 컬렉션에서 고유한 접두사를 생성하는 데 사용됩니다(데이터 충돌을 방지하기 위한 것임)

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();

    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// 사용한 스토리지 양에 따라 초기에 걷은 보증금 환불

pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    // 첨부된 보증금이 필요한 비용과 비교
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    // 환북 금액

    let refund = attached_deposit - required_cost;

    // 환불 금액이 1 yocto near보다 크면 금액을 전임자에게 환불

    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {

    // 소유자가 가지고 있는 토큰 세트에 토큰을 추가합니다.

    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        // 주어진 계정에 대한 토큰 세트 가져오기
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            // 계정에 토큰이 없으면 순서가 지정되지 않은 새 집합을 만듭니다.

            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    // 계정에 토큰이 없으면 순서가 지정되지 않은 새 집합을 만듭니다.
                    account_id_hash: hash_account_id(&account_id)
                }
                    .try_to_vec()
                    .unwrap(),
            )
        });

        // 세트에 토큰 ID를 삽입합니다.
        tokens_set.insert(token_id);
        // 주어진 account ID에 set를 삽입
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }
}