use anyhow::{Context, Result};
use regex::bytes::Regex;
use std::{fmt, path::PathBuf};

#[derive(Debug, Copy, Clone)]
struct Fragment {
    instrs: [u32; 2],
    magic: [u8; 8],
    entry: u32,
    reloc: u32,
    rom_size: u32,
    ram_size: u32,
}

impl From<[u8; 0x20]> for Fragment {
    fn from(a: [u8; 0x20]) -> Self {
        let ep1 = u32::from_be_bytes(a[00..04].try_into().unwrap());
        let ep2 = u32::from_be_bytes(a[04..08].try_into().unwrap());
        let magic = a[08..16].try_into().unwrap();
        let entry = u32::from_be_bytes(a[16..20].try_into().unwrap());
        let reloc = u32::from_be_bytes(a[20..24].try_into().unwrap());
        let rom_size = u32::from_be_bytes(a[24..28].try_into().unwrap());
        let ram_size = u32::from_be_bytes(a[28..32].try_into().unwrap());

        Self {
            instrs: [ep1, ep2],
            magic,
            entry,
            reloc,
            rom_size,
            ram_size,
        }
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "instr: {:08X} {:08X}", self.instrs[0], self.instrs[1])?;
        writeln!(f, "{:?}", std::str::from_utf8(&self.magic))?;
        writeln!(f, "entry: {:#X}", self.entry)?;
        writeln!(f, "reloc: {:#X}", self.reloc)?;
        writeln!(f, "rom:   {:#X} bytes", self.rom_size)?;
        writeln!(f, "ram:   {:#X} bytes", self.ram_size)
    }
}

#[derive(Debug, Copy, Clone)]
struct FragInfo {
    rom_offset: usize,
    number: u8,
    rom_code: u32,
    bss_size: u32,
}

impl FragInfo {
    fn new(rom_offset: usize, frag: Fragment) -> Self {
        let rom_code = frag.rom_size - frag.reloc;
        let bss_size = frag.ram_size - rom_code;

        let number = extract_number_from_j(frag.instrs[0]);

        Self {
            number,
            rom_offset,
            rom_code,
            bss_size,
        }
    }
}

pub fn find_fragments(p: PathBuf, print_seg_info: bool) -> Result<()> {
    let rom = std::fs::read(p).context("reading ROM")?;
    let re = Regex::new("FRAGMENT").unwrap();

    for mat in re.find_iter(&rom) {
        let start = mat.start() - 8;
        if start % 4 == 0 {
            let header: [u8; 0x20] = rom[start..start + 0x20].try_into().unwrap();
            let frag = Fragment::from(header);
            let info = FragInfo::new(start, frag);
            println!(
                "{:#X} - Frag #{} -> {:#X}",
                info.rom_offset,
                info.number,
                info.rom_offset + frag.rom_size as usize
            );
            if print_seg_info {
                println!("\t{:x} code bytes + {:x} bss", info.rom_code, info.bss_size);
            }
        } else {
            // let hdr: [u8; 0x20] = header.try_into().unwrap();
            // let test = Fragment::from(hdr);
            // println!("{test}");
        }
    }

    Ok(())
}

fn extract_number_from_j(op: u32) -> u8 {
    const ADDR_MASK: u32 = (1 << 26) - 1;
    const NUM_MASK: u32 = 0x0FF00000 >> 2;

    let frag = ((op & (ADDR_MASK & NUM_MASK)) >> 18) - 0x10;

    return frag as u8;
}
