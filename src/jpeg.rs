use std::path::PathBuf;

use anyhow::{Context, Error, Result};
use regex::bytes::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct PresJpeg {
    magic: [u8; 8],
    offset: u32,
    unk: u32,
}

impl TryFrom<&[u8]> for PresJpeg {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let magic = value[0..8].try_into()?;
        let offset = u32::from_be_bytes(value[8..12].try_into()?);
        let unk = u32::from_be_bytes(value[12..16].try_into()?);

        Ok(Self { magic, offset, unk })
    }
}

pub fn find_presjpeg(p: PathBuf) -> Result<()> {
    let rom = std::fs::read(p).context("reading ROM")?;
    let re = Regex::new("PRESJPEG").unwrap();
    let jpeg_end = b"\xFF\xD9";

    for mat in re.find_iter(&rom) {
        let start = mat.start();
        let header =
            PresJpeg::try_from(&rom[start..start + 0x10]).context("reading PRESJPEG header")?;
        let jpeg_start = start + header.offset as usize;
        let size = &rom[jpeg_start..]
            .windows(2)
            .take_while(|c| c != jpeg_end)
            .count()
            + header.offset as usize;

        println!("{:#x} - presjpeg -> {:#x}", start, start + size);
    }

    Ok(())
}
