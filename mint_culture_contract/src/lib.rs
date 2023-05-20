use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;


mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod internal;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // 컨트랙트 소유자
    pub owner_id: AccountId,
    // 주어진 계정에 관한 모든 토큰 ID를 keep track 함.
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    // 토큰 ID에 관한 토큰 구조체를 추적
    pub  tokens_by_id: LookupMap<TokenId, Token>,
    // 토큰에 ID에 관한 토큰 메타데이터를 추적함
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    // 계약에 관한 메타데이터를 추적함
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Mint Culture NFT Collect".to_string(),
                symbol: "MintCulture".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {

        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataById.try_to_vec().unwrap()),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata)
            ),
        };
        this
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{ accounts, VMContextBuilder };
    use near_sdk::{ testing_env, Balance} ;
    use super::*;


    const ATTACHED_VALUE: Balance = 100_000_000_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let metadata = NFTContractMetadata {
            spec: "NFT 1.0.0".to_string(),
            name: "TEST_MINT_CULTURE_CONTRACT".to_string(),
            symbol: "TEST_MINT_CULTURE_CONTRACT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };

        let contract = Contract::new(accounts(1).into(), metadata.clone());

        testing_env!(context.is_view(true).build());
        assert_eq!(contract.owner_id, accounts(1));
        assert_eq!(contract.metadata.get().unwrap().spec, metadata.spec);
        assert_eq!(contract.metadata.get().unwrap().name, metadata.name);
        assert_eq!(contract.metadata.get().unwrap().symbol, metadata.symbol);
    }

    /// TEST for enumeration.rs
    #[test]
    fn test_nft_mint_to_myself() {
        let mut context = get_context(accounts(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = Contract::new_default_meta(accounts(1));

        let token_meta_data = TokenMetadata {
            streamer_name: "이름".to_string(),
            title: "제목".to_string(),
            serial_number: "1111222233334444".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: "".to_string(),
        };

        contract.nft_mint(
            "TOKEN_ID_1".to_string(),
            token_meta_data,
            accounts(1),
        );

        let count = contract.nft_total_supply();

        assert_eq!(count, U128(1));

    }

    #[test]
    fn test_nft_mint_to_other() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());

        let mut contract = Contract::new_default_meta(accounts(1));

        let token_meta_data = TokenMetadata {
            streamer_name: "이름".to_string(),
            title: "제목".to_string(),
            serial_number: "1111222233334444".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: "".to_string(),
        };

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(2))
            .build());

        contract.nft_mint(
            "TOKEN_ID_1".to_string(),
            token_meta_data,
            accounts(2),
        );

        let total_count = contract.nft_total_supply();
        let index1_count = contract.nft_supply_for_owner(accounts(1));
        let index2_count = contract.nft_supply_for_owner(accounts(2));

        assert_eq!(total_count, U128(1));
        assert_eq!(index1_count, U128(0));
        assert_eq!(index2_count, U128(1));
    }

    /// TEST for enumeration.rs

    #[test]
    fn test_nft_total_supply() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = Contract::new_default_meta(accounts(1));
        let count = contract.nft_total_supply();

        assert_eq!(count, U128(0));
    }
    #[test]
    fn test_nft_supply_for_owner() {
        let mut context = get_context(accounts(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = Contract::new_default_meta(accounts(1));
        let token_metadata = TokenMetadata {
            streamer_name: "스트리머네임".to_string(),
            title: "제목".to_string(),
            serial_number: "1111222233334444".to_string(),
            media: "https://주소".to_string(),
            media_hash: None,
            description: None,
            issued_at: "오늘".to_string(),
        };
        contract.nft_mint(
            "TOKEN_ID_1".to_string(),
            token_metadata,
            accounts(1)
        );

        let total_count = contract.nft_total_supply();

        let index1_count = contract.nft_supply_for_owner(accounts(1));
        let index2_count = contract.nft_supply_for_owner(accounts(2));

        assert_eq!(total_count, U128(1));
        assert_eq!(index1_count, U128(1));
        assert_eq!(index2_count, U128(0));
    }

    #[test]
    fn test_nft_tokens() {
        let mut contract = Contract::new_default_meta(accounts(1));

        let token_meta_data = TokenMetadata {
            streamer_name: "이름".to_string(),
            title: "제목".to_string(),
            serial_number: "1111222233334444".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: "".to_string(),
        };

        let mut context = get_context(accounts(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(1))
            .build());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(2))
            .build());

        let clone_token_meta_data = TokenMetadata {
            streamer_name: token_meta_data.streamer_name.clone(),
            title: token_meta_data.title.clone(),
            serial_number: "1111222233334445".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: token_meta_data.issued_at.clone()
        };


        contract.nft_mint(
            "TOKEN_ID_1".to_string(),
            token_meta_data,
            accounts(1),
        );

        contract.nft_mint(
            "TOKEN_ID_2".to_string(),
            clone_token_meta_data,
            accounts(2),
        );

        let vec = contract.nft_tokens(None, None);

        assert_eq!("TOKEN_ID_1", vec[0].token_id);
        assert_eq!("TOKEN_ID_2", vec[1].token_id);
    }

    #[test]
    fn test_nft_tokens_for_owner() {
        let mut contract = Contract::new_default_meta(accounts(1));

        let token_meta_data = TokenMetadata {
            streamer_name: "이름".to_string(),
            title: "제목".to_string(),
            serial_number: "1111222233334444".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: "".to_string(),
        };

        let mut context = get_context(accounts(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(1))
            .build());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ATTACHED_VALUE)
            .predecessor_account_id(accounts(2))
            .build());

        let clone_token_meta_data = TokenMetadata {
            streamer_name: token_meta_data.streamer_name.clone(),
            title: token_meta_data.title.clone(),
            serial_number: "1111222233334445".to_string(),
            media: "https://mint.culture.com".to_string(),
            media_hash: None,
            description: None,
            issued_at: token_meta_data.issued_at.clone()
        };

        contract.nft_mint(
            "TOKEN_ID_1".to_string(),
            token_meta_data,
            accounts(1),
        );

        contract.nft_mint(
            "TOKEN_ID_2".to_string(),
            clone_token_meta_data,
            accounts(2),
        );

        let vec = contract.nft_tokens_for_owner(accounts(2), None, None);

        assert_eq!("TOKEN_ID_2", vec[0].token_id);
    }

    /// internal.rs

    #[test]
    fn test_internal_add_token_to_owner() {
        let accountId = accounts(1);
        let tokenID = "Token_1".to_string();

        let mut contract = Contract::new_default_meta(accounts(1));
        contract.internal_add_token_to_owner(&accountId, &tokenID);

        let _token_id = contract.tokens_per_owner.get(&accountId).unwrap();
        assert_eq!(tokenID, _token_id.to_vec()[0]);
        // assert_eq!("Token_2".to_string(), _token_id.to_vec()[0]);
    }

    #[test]
    fn test_internal_remove_token_from_owner() {
        let accountId = accounts(1);
        let tokenID = "Token_1".to_string();

        let mut contract = Contract::new_default_meta(accounts(1));
        contract.internal_add_token_to_owner(&accountId, &tokenID);

        contract.internal_remove_token_from_owner(&accountId, &tokenID);

        let is_none = contract.tokens_per_owner.get(&accountId).is_none();

        assert_eq!(is_none, true);
    }

}