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
    pub set_type: SetType,
    pub slot: EquipSlot,
    pub defense: u16,

    pub model_id_1: u16,
    pub model_id_2: u16,
    pub icon_color: u16,
    pub icon_effect: u8,

    pub rarity: u8,
    pub cost: u32,

    pub fire_res: i8,
    pub water_res: i8,
    pub ice_res: i8,
    pub thunder_res: i8,
    pub dragon_res: i8,

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
#[br(little, magic = b"\x01\x10\x09\x18\x79\x00")]
#[derive(Debug, Serialize)]
pub struct EqCrt {
    #[br(temp)]
    count: u32,

    #[br(count = count)]
    pub entries: Vec<EqCrtEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct EqCrtEntry {
    pub equipment_slot: EquipSlot,
    pub equipment_id: u16,
    pub unlock_item_id: u16,
    pub unlock_monster_id: u32,
    pub unlock_story_id: u32,

    pub unk1: u32,
    pub unk2: u32,

    pub item_1_id: u16,
    pub item_1_count: u8,
    pub item_2_id: u16,
    pub item_2_count: u8,
    pub item_3_id: u16,
    pub item_3_count: u8,
    pub item_4_id: u16,
    pub item_4_count: u8,

    pub unk3: u32,
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
pub enum EquipSlot {
    Head = 0,
    Chest,
    Arms,
    Waist,
    Legs,
    Charm,
}

#[binread]
#[br(little, repr = u8)]
#[derive(Debug, Serialize)]
pub enum SetType {
    Normal = 0,
    FullSet,
    Layered,
    LayeredFullSet,
}
