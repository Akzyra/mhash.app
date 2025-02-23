#![allow(dead_code)]

use binrw::file_ptr::parse_from_iter;
use binrw::{NullString, binread};
use serde::Serialize;
use std::fmt::Debug;
use std::io::SeekFrom;

pub fn clean_string(s: &str) -> String {
    s.replace("<ICON ALPHA>", " α")
        .replace("<ICON BETA>", " β")
        .replace("<ICON GAMMA>", " γ")
        .replace("\r\n", "\n")
}

#[binread]
#[br(little, repr = u32)]
#[derive(Debug, Serialize)]
pub enum Language {
    Japanese = 0,
    English = 1,
    French = 2,
    Spanish = 3,
    German = 4,
    Italian = 5,
    Korean = 6,
    ChineseTrad = 7,
    ChineseSimple = 8,
    Russian = 10,
    Polish = 11,
    Brazilian = 21,
    Arabic = 22,
}

#[binread]
#[br(
    little,
    magic = b"GMD\0\x02\x03\x01\0",
    assert(key_count == string_count),
    assert(name.len() == name_length as usize),
    assert(keys.len() == key_count as usize),
    assert(strings.len() == string_count as usize),
)]
#[derive(Debug, Serialize)]
pub struct Gmd {
    pub language: Language,
    pub zero1: u32,
    pub zero2: u32,

    pub key_count: u32,
    pub string_count: u32,
    pub key_block_size: u32,
    pub string_block_size: u32,

    #[br(temp)]
    name_length: u32,

    // TODO: nicer string convert
    #[br(map = |x: NullString| String::from_utf8_lossy(&x.0).into_owned())]
    pub name: String,

    #[br(count = key_count)]
    pub info_entries: Vec<GmdInfoEntry>,

    #[br(count = 0x100)]
    pub unk_block: Vec<i64>,

    // TODO: nicer string convert
    #[br(
        parse_with = parse_from_iter(info_entries.iter().map(|x| x.key_offset)),
        map = |x: Vec<NullString>| x.iter().map(|y| String::from_utf8_lossy(&y).into_owned()).collect::<Vec<String>>().to_owned(),
        restore_position,
    )]
    pub keys: Vec<String>,

    // TODO: nicer string convert
    #[br(
        seek_before(SeekFrom::Current(key_block_size.into())),
        count = string_count,
        map = |x: Vec<NullString>| x.iter().map(|y| String::from_utf8_lossy(&y).into_owned()).collect::<Vec<String>>().to_owned(),
    )]
    pub strings: Vec<String>,
}

#[binread]
#[br(little)]
#[derive(Debug, Serialize)]
pub struct GmdInfoEntry {
    pub index: u32,
    pub hash1: u32,
    pub hash2: u32,

    pub filler: u32, // CDCDCDCD

    pub key_offset: u64,
    pub list_link: u64,
}

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
    pub slot: Slot,
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

    pub gender: Gender,
    pub set_group: u16,

    pub gmd_name_index: u16,
    pub gmd_desc_index: u16,

    #[br(map = |x: u8| x > 0)]
    pub is_permanent: bool,
}

#[binread]
#[br(little, repr = u32)]
#[derive(Debug, Serialize)]
pub enum Gender {
    Invalid = 0,
    Male,
    Female,
    Unisex,
}

#[binread]
#[br(little, repr = u8)]
#[derive(Debug, Serialize)]
pub enum Slot {
    Head = 0,
    Chest,
    Arms,
    Waist,
    Legs,
    Charm,
}
