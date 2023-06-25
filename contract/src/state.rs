use cw_storage_plus::Item;
use schemars::JsonSchema;
use rs_merkle::{
    MerkleTree, 
    algorithms::Sha256
};
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Binary, Coin};
use cosmwasm_schema::cw_serde;

pub const DEPOSITS: Item<Vec<Deposit>> = Item::new("deposits");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");

pub const MERKLE_ROOT_PREFIX: &str = "vault";
pub const TREE: Item<Binary> = Item::new("merkle_Tree");

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub struct BalanceTree {
  pub tree: MerkleTree::<Sha256>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit { 
  pub addr: String,
  pub amount: Vec<Coin>,
}

#[cw_serde]
pub struct Config {
    pub arbiter: Addr,
    pub source: Addr,
}