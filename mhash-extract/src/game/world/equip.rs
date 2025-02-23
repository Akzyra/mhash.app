#![allow(dead_code)]

use crate::game::bool_u8;
use binrw::binread;
use serde::Serialize;
use std::fmt::Debug;

#[binread]
#[br(little, magic = b"\x01\x10\x09\x18\x5f\x00")]
#[derive(Debug, Serialize)]
pub struct AmDat {
    #[br(temp)]
    count: u32,

    #[br(count = count)]
    pub entries: Vec<AmDatEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct AmDatEntry {
    pub equipment_id: u32,
    pub order: u16,
    pub variant: u8,
    pub set_id: u16,
    pub set_type: u8,
    pub slot: AmDatSlot,
    pub defense: u16,

    pub model_id_1: u16,
    pub model_id_2: u16,
    pub icon_color: u16,
    pub icon_effect: u8,

    pub rarity: u8,
    pub cost: u32,

    pub fire_res: u8,
    pub water_res: u8,
    pub ice_res: u8,
    pub thunder_res: u8,
    pub dragon_res: u8,

    pub deco_count: u8,
    pub deco_1: u8,
    pub deco_2: u8,
    pub deco_3: u8,

    pub set_skill_id: u16,
    pub set_skill_level: u8,
    pub set_skill_2_id: u16,
    pub set_skill_2_level: u8,

    pub skill_1_id: u16,
    pub skill_1_level: u8,
    pub skill_2_id: u16,
    pub skill_2_level: u8,
    pub skill_3_id: u16,
    pub skill_3_level: u8,

    pub gender: AmDatGender,
    pub set_group: u16,

    pub gmd_name_index: u16,
    pub gmd_desc_index: u16,

    #[br(parse_with = bool_u8)]
    pub is_permanent: bool,
}

#[binread]
#[br(little, repr = u32)]
#[derive(Debug, Serialize)]
pub enum AmDatGender {
    Invalid = 0,
    Male,
    Female,
    Unisex,
}

#[binread]
#[br(little, repr = u8)]
#[derive(Debug, Serialize)]
pub enum AmDatSlot {
    Head = 0,
    Chest,
    Arms,
    Waist,
    Legs,
    Charm,
}
