use cosmwasm_std::Addr;
use cw_storage_plus::{Map};

pub const STATE: Map<Addr, Addr> = Map::new("pairs");
