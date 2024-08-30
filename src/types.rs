pub mod candid {
    use candid::{CandidType, Deserialize, Nat};
    use ic_cdk::api::call::RejectionCode;
    use ic_cdk::api::management_canister::http_request::HttpHeader;
    use serde::Serialize;
    use std::iter;
    use thiserror::Error;

    #[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Default)]
    pub enum BlockTag {
        #[default]
        Latest,
        Finalized,
        Safe,
        Earliest,
        Pending,
        Number(Nat),
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
    pub struct Block {
        #[serde(rename = "baseFeePerGas")]
        pub base_fee_per_gas: Nat,
        pub number: Nat,
        pub difficulty: Nat,
        #[serde(rename = "extraData")]
        pub extra_data: String,
        #[serde(rename = "gasLimit")]
        pub gas_limit: Nat,
        #[serde(rename = "gasUsed")]
        pub gas_used: Nat,
        pub hash: String,
        #[serde(rename = "logsBloom")]
        pub logs_bloom: String,
        pub miner: String,
        #[serde(rename = "mixHash")]
        pub mix_hash: String,
        pub nonce: Nat,
        #[serde(rename = "parentHash")]
        pub parent_hash: String,
        #[serde(rename = "receiptsRoot")]
        pub receipts_root: String,
        #[serde(rename = "sha3Uncles")]
        pub sha3_uncles: String,
        pub size: Nat,
        #[serde(rename = "stateRoot")]
        pub state_root: String,
        #[serde(rename = "timestamp")]
        pub timestamp: Nat,
        #[serde(rename = "totalDifficulty")]
        pub total_difficulty: Nat,
        #[serde(default)]
        pub transactions: Vec<String>,
        #[serde(rename = "transactionsRoot")]
        pub transactions_root: Option<String>,
        #[serde(default)]
        pub uncles: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
    pub struct LogEntry {
        pub address: String,
        pub topics: Vec<String>,
        pub data: String,
        #[serde(rename = "blockNumber")]
        pub block_number: Option<Nat>,
        #[serde(rename = "transactionHash")]
        pub transaction_hash: Option<String>,
        #[serde(rename = "transactionIndex")]
        pub transaction_index: Option<Nat>,
        #[serde(rename = "blockHash")]
        pub block_hash: Option<String>,
        #[serde(rename = "logIndex")]
        pub log_index: Option<Nat>,
        #[serde(default)]
        pub removed: bool,
    }

    #[derive(Debug, Clone, Deserialize, PartialEq, Eq, CandidType)]
    pub struct GetLogsArgs {
        #[serde(rename = "fromBlock")]
        pub from_block: Option<BlockTag>,
        #[serde(rename = "toBlock")]
        pub to_block: Option<BlockTag>,
        pub addresses: Vec<String>,
        pub topics: Option<Vec<Vec<String>>>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
    pub struct FeeHistoryArgs {
        #[serde(rename = "blockCount")]
        pub block_count: u128,
        #[serde(rename = "newestBlock")]
        pub newest_block: BlockTag,
        #[serde(rename = "rewardPercentiles")]
        pub reward_percentiles: Option<Vec<u8>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, CandidType)]
    pub struct FeeHistory {
        #[serde(rename = "oldestBlock")]
        pub oldest_block: Nat,
        #[serde(rename = "baseFeePerGas")]
        pub base_fee_per_gas: Vec<Nat>,
        #[serde(default)]
        #[serde(rename = "gasUsedRatio")]
        pub gas_used_ratio: Vec<f64>,
        pub reward: Vec<Vec<Nat>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, CandidType)]
    pub struct TransactionReceipt {
        #[serde(rename = "blockHash")]
        pub block_hash: String,
        #[serde(rename = "blockNumber")]
        pub block_number: Nat,
        #[serde(rename = "effectiveGasPrice")]
        pub effective_gas_price: Nat,
        #[serde(rename = "gasUsed")]
        pub gas_used: Nat,
        pub status: Nat,
        #[serde(rename = "transactionHash")]
        pub transaction_hash: String,
        #[serde(rename = "contractAddress")]
        pub contract_address: Option<String>,
        pub from: String,
        pub logs: Vec<LogEntry>,
        #[serde(rename = "logsBloom")]
        pub logs_bloom: String,
        pub to: String,
        #[serde(rename = "transactionIndex")]
        pub transaction_index: Nat,
        pub r#type: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
    pub struct GetTransactionCountArgs {
        pub address: String,
        pub block: BlockTag,
    }

    #[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
    pub enum SendRawTransactionStatus {
        Ok(Option<String>),
        InsufficientFunds,
        NonceTooLow,
        NonceTooHigh,
    }

    pub type RpcResult<T> = Result<T, RpcError>;

    #[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
    pub enum MultiRpcResult<T> {
        Consistent(RpcResult<T>),
        Inconsistent(Vec<(RpcService, RpcResult<T>)>),
    }

    impl<T> MultiRpcResult<T> {
        pub fn iter(&self) -> Box<dyn Iterator<Item = &RpcResult<T>> + '_> {
            match self {
                Self::Consistent(result) => Box::new(iter::once(result)),
                Self::Inconsistent(results) => {
                    Box::new(results.iter().map(|(_service, result)| result))
                }
            }
        }
    }

    #[derive(Clone, Error, Debug, PartialEq, Eq, CandidType, Deserialize)]
    pub enum RpcError {
        #[error("Provider error: {0}")]
        ProviderError(ProviderError),
        #[error("HTTP outcall error: {0}")]
        HttpOutcallError(HttpOutcallError),
        #[error("JSON-RPC error: {0}")]
        JsonRpcError(JsonRpcError),
        #[error("Validation error: {0}")]
        ValidationError(ValidationError),
    }

    impl RpcError {
        pub(crate) fn from_rejection(code: RejectionCode, message: String) -> Self {
            Self::HttpOutcallError(HttpOutcallError::IcError { code, message })
        }
    }

    #[derive(Clone, Error, Debug, PartialEq, Eq, CandidType, Deserialize)]
    pub enum ProviderError {
        #[error("No permission to call this provider")]
        NoPermission,
        #[error("Not enough cycles, expected {expected}, received {received}")]
        TooFewCycles { expected: u128, received: u128 },
        #[error("Provider not found")]
        ProviderNotFound,
        #[error("Missing required provider")]
        MissingRequiredProvider,
    }

    #[derive(Clone, Error, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize)]
    pub enum HttpOutcallError {
        #[error("IC error (code: {code:?}): {message}")]
        IcError {
            code: RejectionCode,
            message: String,
        },
        #[error("Invalid HTTP JSON-RPC response: status {status}, body: {body}, parsing error: {parsing_error:?}")]
        InvalidHttpJsonRpcResponse {
            status: u16,
            body: String,
            #[serde(rename = "parsingError")]
            parsing_error: Option<String>,
        },
    }

    #[derive(
        Clone, Error, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Serialize, Deserialize,
    )]
    #[error("JSON-RPC error (code: {code}): {message}")]
    pub struct JsonRpcError {
        pub code: i64,
        pub message: String,
    }

    #[derive(Clone, Error, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize)]
    pub enum ValidationError {
        #[error("Custom: {0}")]
        Custom(String),
        #[error("Invalid hex: {0}")]
        InvalidHex(String),
        #[error("Invalid URL: {0}")]
        UrlParseError(String),
        #[error("Host not allowed: {0}")]
        HostNotAllowed(String),
        #[error("Credential path not allowed")]
        CredentialPathNotAllowed,
        #[error("Credential header not allowed")]
        CredentialHeaderNotAllowed,
    }

    #[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
    pub enum RpcServices {
        Custom {
            #[serde(rename = "chainId")]
            chain_id: u64,
            services: Vec<RpcApi>,
        },
        EthMainnet(Option<Vec<EthMainnetService>>),
        EthSepolia(Option<Vec<EthSepoliaService>>),
    }

    #[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, CandidType)]
    pub struct RpcApi {
        pub url: String,
        pub headers: Option<Vec<HttpHeader>>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, CandidType)]
    pub enum RpcService {
        Custom(RpcApi),
        EthMainnet(EthMainnetService),
        EthSepolia(EthSepoliaService),
    }

    #[derive(
        Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, CandidType,
    )]
    pub enum EthMainnetService {
        Alchemy,
        Ankr,
        BlockPi,
        PublicNode,
        Cloudflare,
    }

    #[derive(
        Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, CandidType,
    )]
    pub enum EthSepoliaService {
        Alchemy,
        Ankr,
        BlockPi,
        PublicNode,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default, CandidType, Deserialize)]
    pub struct RpcConfig {
        #[serde(rename = "responseSizeEstimate")]
        pub response_size_estimate: Option<u64>,
    }
}
