#![allow(dead_code)]

use crate::game::bool_u8;
use binrw::binread;
use serde::Serialize;
use std::fmt::Debug;

#[binread]
#[br(little, magic = b"\x01\x10\x09\x18\xbc\x00")]
#[derive(Debug, Serialize)]
pub struct SklDat {
    #[br(temp)]
    count: u32,

    #[br(count = count)]
    pub entries: Vec<SklDatEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct SklDatEntry {
    pub skl_pt_id: u16,
    pub skill_level: u8,

    pub unlock_skill_1_id: u32,
    pub unlock_skill_2_id: u32,
    pub unlock_skill_3_id: u32,
    pub unlock_skill_4_id: u32,
    pub unlock_skill_5_id: u32,
    pub unlock_skill_6_id: u32,

    pub param_0: u16,
    pub param_1: u16,
    pub param_2: u16,
    pub param_3: u16,
}

#[binread]
#[br(little, magic = b"\x01\x10\x09\x18\x5e\x00")]
#[derive(Debug, Serialize)]
pub struct SklPtDat {
    #[br(temp)]
    count: u32,

    #[br(count = count)]
    pub entries: Vec<SklPtDatEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct SklPtDatEntry {
    #[br(parse_with = bool_u8)]
    pub is_set_skill: bool,
    pub icon_color_id: u8,
}
