use crate::*;
use near_sdk:: {CryptoHash};
use std::mem::size_of;
use near_sdk::Promise;

pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}
pub(crate) fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_account_ids: I,
) -> Promise
    where
        I: Iterator<Item = &'a AccountId>,
{
    let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

// 스토리지 컬렉션에서 고유한 접두사를 생성하는 데 사용됩니다(데이터 충돌을 방지하기 위한 것임)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();

    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Error: 1 옥토니어 첨부가 필요함"
    )
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
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        approval_id: Option<u64>,
        memo: Option<String>
    ) -> Token {
        let token = self.tokens_by_id.get(token_id).expect("No Exist Token");

        if sender_id != &token.owner_id {

            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }
            if let Some(enforced_approval_id) = approval_id {
                //get the actual approval ID
                let actual_approval_id = token
                    .approved_account_ids
                    .get(sender_id)
                    //if the sender isn't in the map, we panic
                    .expect("Sender is not approved account");

                //make sure that the actual approval ID is the same as the one provided
                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval_id {} is different from the given approval_id {}",
                    actual_approval_id, enforced_approval_id,
                );
            }
        }

        assert_ne!(
            &token.owner_id, receiver_id,
            "토큰 소유자와 토큰 수신자는 달라야 합니다."
        );
        // 보내는 사람 토큰 목록에서 제거
        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        // 받는 사람 토큰 목록에 추가
        self.internal_add_token_to_owner(receiver_id, token_id);


        let new_token = Token {
            owner_id: receiver_id.clone(),
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
        };

        self.tokens_by_id.insert(token_id, &new_token);
        if let Some(memo) = memo {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }
        token
    }
    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .expect("Token should be owned by the sender");

        tokens_set.remove(token_id);
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }
}