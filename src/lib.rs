#![allow(unused)]
use std::collections::BTreeMap;

use miniscript::bitcoin::{taproot::Signature, Network, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub accounts: Vec<Account>,
    pub network: Network,
    /// App proprietary metadata (settings, configuration, etc..)
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub proprietary: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey>,
    pub active: bool,
    pub receive_index: Option<u64>,
    pub change_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub keys: BTreeMap<miniscript::bitcoin::bip32::Fingerprint, Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<bip329::Labels>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub psbts: Vec<miniscript::bitcoin::Psbt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip39_mnemonic: Option<bip39::Mnemonic>,
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub proprietary: serde_json::Map<String, serde_json::Value>,
}

impl Account {
    pub fn new(descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey>) -> Self {
        Self {
            name: None,
            description: None,
            descriptor,
            active: true,
            receive_index: None,
            change_index: None,
            timestamp: None,
            keys: BTreeMap::new(),
            labels: None,
            transactions: Vec::new(),
            psbts: Vec::new(),
            bip39_mnemonic: None,
            proprietary: serde_json::Map::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub key: miniscript::bitcoin::bip32::Fingerprint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<KeyRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_status: Option<KeyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip85_derivation_path: Option<miniscript::bitcoin::bip32::DerivationPath>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyRole {
    /// Key to be used in normal spending condition
    Main,
    /// Key that will be used for recover in case loss of main key(s)
    Recovery,
    /// Key that wil inherit coins if main user disapear
    Inheritance,
    /// Key that will cosign a spend in order to enforce some policy
    Cosigning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    /// Main user
    Internal,
    /// Heirs or friends
    External,
    /// Service the user pay for
    ThirdParty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyStatus {
    Active,
    Inactive,
    Revoked,
}
