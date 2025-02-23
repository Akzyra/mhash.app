use binrw::{BinRead, BinResult};

pub mod world;

#[binrw::parser(reader, endian)]
fn bool_u8() -> BinResult<bool> {
    let val = u8::read_options(reader, endian, ())?;
    Ok(val > 0)
}
