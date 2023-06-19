use cosmwasm_std::{
    Addr,
    Binary
};
use cw_storage_plus::{
    Item,
};
use rs_merkle::{
    MerkleTree, 
    algorithms::Sha256
};
use cosmwasm_schema::cw_serde;


pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");

pub const MERKLE_ROOT_PREFIX: &str = "vault";
pub const TREE: Item<Binary> = Item::new("merkle_Tree");

pub struct BalanceTree {
  pub tree: MerkleTree::<Sha256>,
}
