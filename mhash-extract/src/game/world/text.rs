#![allow(dead_code)]

use binrw::file_ptr::parse_from_iter_with;
use binrw::helpers::count_with;
use binrw::{BinRead, BinResult, NullString, binread};
use serde::Serialize;
use std::fmt::Debug;
use std::io::SeekFrom;

pub fn clean_string(s: &str) -> String {
    s.replace("<ICON ALPHA>", " α")
        .replace("<ICON BETA>", " β")
        .replace("<ICON GAMMA>", " γ")
        .replace("\r\n", "\n")
}

#[binrw::parser(reader, endian)]
fn null_uf8() -> BinResult<String> {
    let ns = NullString::read_options(reader, endian, ())?;
    let s = String::from_utf8_lossy(ns.as_slice()).into();
    Ok(s)
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
    pub language: GmdLanguage,
    pub zero1: u32,
    pub zero2: u32,

    pub key_count: u32,
    pub string_count: u32,
    pub key_block_size: u32,
    pub string_block_size: u32,

    #[br(temp)]
    name_length: u32,

    #[br(parse_with = null_uf8)]
    pub name: String,

    #[br(count = key_count)]
    pub info_entries: Vec<GmdInfoEntry>,

    #[br(count = 0x100)]
    pub unk_block: Vec<i64>,

    #[br(
        parse_with = parse_from_iter_with(
            info_entries.iter().map(|x| x.key_offset),
            null_uf8,
        ),
        restore_position,
    )]
    pub keys: Vec<String>,

    #[br(
        seek_before(SeekFrom::Current(key_block_size.into())),
        parse_with = count_with(string_count as usize, null_uf8),
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
#[br(little, repr = u32)]
#[derive(Debug, Serialize)]
pub enum GmdLanguage {
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
