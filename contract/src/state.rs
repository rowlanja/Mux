use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

pub const MERKLE_ROOT_PREFIX: &str = "vault";
pub const MERKLE_ROOT: Map<u8, String> = Map::new(MERKLE_ROOT_PREFIX);