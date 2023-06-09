use crate::*;
pub type TokenId = String;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
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
    // Streamer name
    pub streamer_name: String,

    // NFT 이름 -
    pub title: String,

    // 일련번호
    pub serial_number: String,

    // 연결된 미디어의 URL
    pub media: String,
    pub media_hash: Option<Base64VecU8>,

    // NFT 설명 -
    pub description: Option<String>,

    // 토큰 발행 일자
    pub issued_at: String
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // Token 구조체는 메타데이터를 제외한 직접적인 정보를 모두 가짐. Token ID를 전달하기만 하면,
    // 모든 메타데이터에 빠르게 접근 가능
    // tokenMetaById 자료형으로 저장됨.
    pub owner_id: AccountId,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub next_approval_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    // View 호출을 할 때마다 JSON 형태로 다시 보내려는 NFT에 대한 모든 정보를 보유하는 것입니다.
    //  즉, 소유자, 토큰 ID 및 메타데이터를 저장해야 합니다.
    pub token_id: TokenId,

    pub owner_id: AccountId,

    pub metadata: TokenMetadata,

    pub approved_account_ids: HashMap<AccountId, u64>,
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        // 메타데이터 쿼리를 위한 함수
        self.metadata.get().unwrap()
    }
}