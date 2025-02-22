#![allow(dead_code)]

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
    #[br(map = |x: u8| x > 0)]
    pub is_set_skill: bool,
    pub icon_color_id: u8,
}

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
    pub slot: u8,
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

    pub gender: u32,
    pub set_group: u16,

    pub gmd_name_index: u16,
    pub gmd_desc_index: u16,

    #[br(map = |x: u8| x > 0)]
    pub is_permanent: bool,
}
