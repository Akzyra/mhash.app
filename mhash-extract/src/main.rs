use crate::game::world::{SklDat, SklPtDat};
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

    let skill_file = File::open(Path::new(unpacked).join(skill_file_path)).expect("File not found");
    let skill_point_file =
        File::open(Path::new(unpacked).join(skill_point_file_path)).expect("File not found");
    let mut r;

    r = BufReader::new(skill_point_file);
    let skill_point_data = SklPtDat::read(&mut r).expect("read error");
    fs::write(
        "./dump/skill_point_data.json",
        serde_json::to_string_pretty(&skill_point_data).unwrap(),
    )
    .expect("write error");
    println!("skill_point_data: {}", skill_point_data.count);

    r = BufReader::new(skill_file);
    let skill_data = SklDat::read(&mut r).expect("read error");
    fs::write(
        "./dump/skill_data.json",
        serde_json::to_string_pretty(&skill_data).unwrap(),
    )
    .expect("write error");
    println!("skill_data: {}", skill_data.count);
}
