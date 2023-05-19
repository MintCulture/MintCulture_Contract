use crate::*;
pub type TokenId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    /*
        FILL THIS IN
    */
    pub spec: String,              // required, 버전
    pub name: String,              // required, 이름
    pub symbol: String,            // required, 심볼
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, //  reference or media URL에 의해 참조 되어지는 자산으로 연결된 접근?
    pub reference: Option<String>, // URL
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    /*
        FILL THIS IN
    */
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // NFT가 온체인에 저장하려는 추가 항목 -  JSON Stringfy 가능
    pub reference: Option<String>, // 오프체인 JSON 파일의 URL
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    /*
        FILL THIS IN
    */
    // Token 구조체는 메타데이터를 제외한 직접적인 정보를 모두 가짐. Token ID를 전달하기만 하면,
    // 모든 메타데이터에 빠르게 접근 가능
    // tokenMetaById 자료형으로 저장됨.
    pub owner_id: AccountId,
}

//The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    /*
        FILL THIS IN
    */
    // View 호출을 할 때마다 JSON 형태로 다시 보내려는 NFT에 대한 모든 정보를 보유하는 것입니다.
    //  즉, 소유자, 토큰 ID 및 메타데이터를 저장해야 합니다.

    pub token_id: TokenId,

    pub owner_id: AccountId,

    pub metadata: TokenMetadata,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        /*
            FILL THIS IN
        */
        // 메타데이터 쿼리를 위한 함수

        self.metadata.get().unwrap()
    }
}