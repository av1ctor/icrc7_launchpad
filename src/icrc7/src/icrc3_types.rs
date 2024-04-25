use candid::{CandidType, Principal};
use icrc_ledger_types::{
    icrc::generic_value::{Hash, Map, Value},
    icrc1::account::Account,
};

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::{collections::HashMap, convert::From, ops::Deref, string::ToString};

use crate::Transaction;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(Value);

impl AsRef<Value> for Block {
    #[inline]
    fn as_ref(&self) -> &Value {
        &self.0
    }
}

impl Deref for Block {
    type Target = Value;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Map> for Block {
    fn from(value: Map) -> Self {
        Self(Value::Map(value))
    }
}

impl TryFrom<Value> for Block {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => Ok(Self(Value::Map(map))),
            _ => Err("block must be a map value".to_string()),
        }
    }
}

impl Block {
    pub fn new(phash: Hash, tx: Transaction) -> Self {
        let mut block = Map::new();
        block.insert("phash".to_string(), Value::Blob(ByteBuf::from(phash)));
        block.insert("btype".to_string(), Value::Text(tx.op));
        block.insert("ts".to_string(), Value::Nat(tx.ts.into()));

        let mut val = Map::new();
        val.insert("tid".to_string(), Value::Nat(tx.tid.into()));
        if let Some(from) = tx.from {
            val.insert("from".to_string(), account_value(from));
        }
        if let Some(to) = tx.to {
            val.insert("to".to_string(), account_value(to));
        }
        if let Some(spender) = tx.spender {
            val.insert("spender".to_string(), account_value(spender));
        }
        if let Some(exp) = tx.exp {
            val.insert("exp".to_string(), Value::Nat(exp.into()));
        }
        if let Some(meta) = tx.meta {
            val.insert("meta".to_string(), Value::Map(meta));
        }
        if let Some(memo) = tx.memo {
            val.insert("memo".to_string(), Value::Blob(ByteBuf::from(memo)));
        }
        val.insert("ts".to_string(), Value::Nat(tx.ts.into()));
        block.insert("tx".to_string(), Value::Map(val));
        Self(Value::Map(block))
    }

    pub fn into_inner(self) -> Value {
        self.0
    }

    pub fn into_map(self) -> Map {
        match self.0 {
            Value::Map(map) => map,
            _ => unreachable!(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BlockType {
    pub block_type: String,
    pub url: String,
}

fn account_value(Account { owner, subaccount }: Account) -> Value {
    let mut parts = vec![Value::blob(owner.as_slice())];
    if let Some(subaccount) = subaccount {
        parts.push(Value::blob(subaccount.as_slice()));
    }
    Value::Array(parts)
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum IndexType {
    Managed,
    Stable,
    StableTyped,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRange {
    pub start: u128,
    pub length: u128,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveLedgerInfo {
    pub archives: HashMap<Principal, TransactionRange>,
    pub local_ledger_size: u128,
    pub supported_blocks: Vec<BlockType>,
    pub last_index: u128,
    pub first_index: u128,
    pub is_cleaning: bool,
    pub latest_hash: Option<Vec<u8>>,
    pub setting: ArchiveSetting,
}

impl Default for ArchiveLedgerInfo {
    fn default() -> Self {
        Self {
            archives: HashMap::new(),
            local_ledger_size: 0,
            supported_blocks: vec![],
            last_index: 0,
            first_index: 0,
            is_cleaning: false,
            latest_hash: None,
            setting: ArchiveSetting::default(),
        }
    }
}

impl ArchiveLedgerInfo {
    pub fn new(setting: Option<ArchiveSetting>) -> Self {
        let setting = setting.unwrap_or(ArchiveSetting::default());
        Self {
            archives: HashMap::new(),
            local_ledger_size: 0,
            last_index: 0,
            first_index: 0,
            is_cleaning: false,
            latest_hash: None,
            setting,
            supported_blocks: vec![
                BlockType {
                    block_type: "7mint".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
                },
                BlockType {
                    block_type: "7burn".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
                },
                BlockType {
                    block_type: "7xfer".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
                },
                BlockType {
                    block_type: "7update".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
                },
                BlockType {
                    block_type: "37appr".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md"
                        .into(),
                },
                BlockType {
                    block_type: "37appr_coll".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md"
                        .into(),
                },
                BlockType {
                    block_type: "37revoke".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md"
                        .into(),
                },
                BlockType {
                    block_type: "37revoke_coll".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md"
                        .into(),
                },
                BlockType {
                    block_type: "37xfer".into(),
                    url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md"
                        .into(),
                },
            ],
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveSetting {
    pub archive_controllers: Option<Option<Vec<Principal>>>,
    pub archive_cycles: u128,
    pub archive_index_type: IndexType,
    pub max_active_records: u128,
    pub max_archive_pages: u128,
    pub max_records_in_archive_instance: u128,
    pub max_records_to_archive: u128,
    pub settle_to_records: u128,
}

impl Default for ArchiveSetting {
    fn default() -> Self {
        Self {
            archive_controllers: None,
            archive_cycles: 2_000_000_000_000,
            archive_index_type: IndexType::Stable,
            max_active_records: 2000,
            max_archive_pages: 62500,
            max_records_in_archive_instance: 10_000_000,
            max_records_to_archive: 10_000,
            settle_to_records: 1000,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArchiveArg {
    #[serde(rename = "archiveControllers")]
    pub archive_controllers: Option<Option<Vec<Principal>>>,
    #[serde(rename = "archiveCycles")]
    pub archive_cycles: u128,
    #[serde(rename = "archiveIndexType")]
    pub archive_index_type: IndexType,
    #[serde(rename = "maxActiveRecords")]
    pub max_active_records: u128,
    #[serde(rename = "maxArchivePages")]
    pub max_archive_pages: u128,
    #[serde(rename = "maxRecordsInArchiveInstance")]
    pub max_records_in_archive_instance: u128,
    #[serde(rename = "maxRecordsToArchive")]
    pub max_records_to_archive: u128,
    #[serde(rename = "settleToRecords")]
    pub settle_to_records: u128,
}

impl InitArchiveArg {
    pub fn to_archive_setting(self) -> ArchiveSetting {
        ArchiveSetting {
            archive_controllers: self.archive_controllers,
            archive_cycles: self.archive_cycles,
            archive_index_type: self.archive_index_type,
            max_active_records: self.max_active_records,
            max_archive_pages: self.max_archive_pages,
            max_records_in_archive_instance: self.max_records_in_archive_instance,
            max_records_to_archive: self.max_records_to_archive,
            settle_to_records: self.settle_to_records,
        }
    }
}
