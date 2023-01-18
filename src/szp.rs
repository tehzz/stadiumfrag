use std::{fmt, path::PathBuf};

use anyhow::{Context, Result};
use regex::bytes::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct PersSzp {
    magic: [u8; 8],
    offset: u32,
    // RAM size after decompression?
    size: u32,
    size_again: u32,
    num_updates: u32,
    // (update_value, offset) (maybe?)
    updates: Vec<(u32, usize)>,
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

impl TryFrom<&[u8]> for PersSzp {
    type Error = anyhow::Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        const MAGIC_BYTES: &[u8] = b"PERS-SZP";

        let magic = v[0..8].try_into()?;
        if &magic != MAGIC_BYTES {
            anyhow::bail!("Expected <{:x?}> for magic, got <{:x?}>", MAGIC_BYTES, &magic);
        }
        let offset = u32::from_be_bytes(v[0x8..0xC].try_into()?);
        let size = u32::from_be_bytes(v[0xC..0x10].try_into()?);
        let size_again = u32::from_be_bytes(v[0x10..0x14].try_into()?);
        let num_updates = u32::from_be_bytes(v[0x14..0x18].try_into()?);
        let updates = v[0x18..0x18+2*4*num_updates as usize]
            .chunks(8)
            .map(|c| (
                u32::from_be_bytes(c[..4].try_into().unwrap()), 
                u32::from_be_bytes(c[4..].try_into().unwrap()) as usize, 
            )).collect();

        Ok(Self {
            magic,
            offset,
            size,
            size_again,
            num_updates,
            updates
        })
    }
}

pub fn find_pers_szp(p: PathBuf) -> Result<()> {
    let rom = std::fs::read(p).context("reading ROM")?;
    let re = Regex::new("PERS-SZP").unwrap();

    for mat in re.find_iter(&rom) {
        let start = mat.start();
        let header = PersSzp::try_from(&rom[start..])?;
        let data_start = start + header.offset as usize;
        let test = &rom[data_start..data_start + 4];

        print!("{:#X}\t{}\t", start, header);
        if let Ok(s) = std::str::from_utf8(test) {
            println!("{s}");
        } else {
            println!("{:x?}", test);
        }
        if header.num_updates != 0 {
            println!("\tUpdates: {:X?}", header.updates);
        }
    }

    Ok(())
}
