use crate::*;
use crate::internal::refund_deposit;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        // 계약에서 사용 중인 초기 저장소를 측정합니다.
        let initial_storage_usage = env::storage_usage();

        // 소유자 ID를 포함하는 토큰 구조체 지정
        let token = Token {
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
        };

        // 토큰 ID와 토큰 구조체를 삽입하고 토큰이 존재하지 않는지 확인합니다.
        let token_id = &token_id[..];

        assert!(
            self.tokens_by_id.insert(&token_id.to_string(), &token).is_none(),
            "Token already exists"
        );
        // 토큰 ID 및 메타데이터 삽입


        self.token_metadata_by_id.insert(&token_id.to_string(), &metadata);

        // 소유자에게 토큰을 추가하기 위한 내부 메서드를 호출합니다.
        self.internal_add_token_to_owner(&token.owner_id,&token_id.to_string());

        // 사용된 필수 스토리지 계산 - 초기
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // 사용자가 너무 많이 첨부한 경우 초과 저장용량을 환불합니다.

        // 구현 해야함 - internal.rs를 만들어서 구현할 것
        refund_deposit(required_storage_in_bytes);

    }
}