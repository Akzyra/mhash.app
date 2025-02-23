use crate::game::world::{AmDat, Gmd, SklDat, SklPtDat};
use binrw::BinRead;
use binrw::io::BufReader;
use std::fs;
use std::fs::File;
use std::path::Path;

mod game;

fn main() {
    let unpacked = r"H:\MH_WORLD\chunk_combined\";
    let skill_file_path = "common/equip/skill_data.skl_dat";
    let skill_point_file_path = "common/equip/skill_point_data.skl_pt_dat";
    let armor_file_path = "common/equip/armor.am_dat";
    let armor_eng_path = "common/text/steam/armor_eng.gmd";

    let skill_file = File::open(Path::new(unpacked).join(skill_file_path)).expect("File not found");
    let skill_point_file =
        File::open(Path::new(unpacked).join(skill_point_file_path)).expect("File not found");
    let armor_file = File::open(Path::new(unpacked).join(armor_file_path)).expect("File not found");
    let armor_eng_file =
        File::open(Path::new(unpacked).join(armor_eng_path)).expect("File not found");

    let mut r;

    r = BufReader::new(skill_point_file);
    let skill_point_data = SklPtDat::read(&mut r).expect("read error");
    fs::write(
        "./dump/skill_point_data.json",
        serde_json::to_string_pretty(&skill_point_data).unwrap(),
    )
    .expect("write error");
    println!("skill_point_data: {}", skill_point_data.entries.len());

    r = BufReader::new(skill_file);
    let skill_data = SklDat::read(&mut r).expect("read error");
    fs::write(
        "./dump/skill_data.json",
        serde_json::to_string_pretty(&skill_data).unwrap(),
    )
    .expect("write error");
    println!("skill_data: {}", skill_data.entries.len());

    r = BufReader::new(armor_file);
    let armor_data = AmDat::read(&mut r).expect("read error");
    fs::write(
        "./dump/armor_data.json",
        serde_json::to_string_pretty(&armor_data).unwrap(),
    )
    .expect("write error");
    println!("armor_data: {}", armor_data.entries.len());

    r = BufReader::new(armor_eng_file);
    let armor_eng = Gmd::read(&mut r).expect("read error");
    fs::write(
        "./dump/armor_eng.json",
        serde_json::to_string_pretty(&armor_eng).unwrap(),
    )
    .expect("write error");
    println!("armor_eng: {}", armor_eng.string_count);
}
