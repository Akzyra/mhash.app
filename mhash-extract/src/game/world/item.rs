#![allow(dead_code)]

use binrw::file_ptr::parse_from_iter_with;
use binrw::helpers::count_with;
use binrw::{BinRead, BinResult, NullString, binread};
use serde::Serialize;
use std::fmt::Debug;
use std::io::SeekFrom;

#[binread]
#[br(little, magic = b"\x01\x10\x09\x18\xbe\x00")]
#[derive(Debug, Serialize)]
pub struct Itm {
    #[br(temp)]
    count: u32,

    #[br(count = count)]
    pub entries: Vec<ItmEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct ItmEntry {
    pub item_id: u32,
    pub sub_type: u8,
    pub r#type: u32,

    pub rarity: u8,
    pub carry_limit: u8,
    pub unk_limit: u8,
    pub order: u16,
    pub flags: u32,

    pub icon_id: u32,
    pub icon_color: u8,
    pub carry_item: u8,

    pub sell_price: u32,
    pub buy_price: u32,
}
