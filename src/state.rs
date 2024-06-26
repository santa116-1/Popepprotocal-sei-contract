use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BookEntry {
    pub id: u64,
    pub owner: Addr,
    pub cw20_address: Addr,
    pub payment_cw20_address: Addr,
    pub amount: Uint128,
    pub price: Uint128,
}

pub const BOOK_ENTRY_SEQ: Item<u64> = Item::new("book_entry_seq");
pub const BOOK_LIST: Map<u64, BookEntry> = Map::new("book_list");
