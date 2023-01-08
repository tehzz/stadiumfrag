use std::{fmt, path::PathBuf};

use anyhow::{Context, Result};
use regex::bytes::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct PersSzp {
    magic: [u8; 8],
    offset: u32,
    size: u32,
    size_again: u32,
}

impl fmt::Display for PersSzp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let magic = std::str::from_utf8(&self.magic).expect("valid ASCII magic");
        write!(
            f,
            "{} - {:#x} for {:#x} bytes",
            magic, self.offset, self.size
        )?;
        if self.size != self.size_again {
            write!(f, "NO MATCH {:#x}", self.size_again)
        } else {
            Ok(())
        }
    }
}

impl TryFrom<[u8; 0x18]> for PersSzp {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 0x18]) -> Result<Self, Self::Error> {
        let magic = value[0..8].try_into()?;
        let offset = u32::from_be_bytes(value[8..12].try_into()?);
        let size = u32::from_be_bytes(value[12..16].try_into()?);
        let size_again = u32::from_be_bytes(value[16..20].try_into()?);

        Ok(Self {
            magic,
            offset,
            size,
            size_again,
        })
    }
}

pub fn find_pers_szp(p: PathBuf) -> Result<()> {
    let rom = std::fs::read(p).context("reading ROM")?;
    let re = Regex::new("PERS-SZP").unwrap();

    for mat in re.find_iter(&rom) {
        let start = mat.start();
        let raw: [u8; 0x18] = rom[start..start + 0x18].try_into()?;
        let header = PersSzp::try_from(raw)?;
        let data_start = start + header.offset as usize;
        let test = &rom[data_start..data_start + 4];

        print!("{:#X}\t{}\t", start, header);
        if let Ok(s) = std::str::from_utf8(test) {
            println!("{s}");
        } else {
            println!("{:x?}", test);
        }
    }

    Ok(())
}
