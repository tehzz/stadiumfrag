use anyhow::{bail, Context, Result};
use bitstream_io::{BigEndian, BitRead, BitReader, ByteRead, ByteReader};
use regex::bytes::Regex;
use std::{borrow::Cow, cmp::Ordering, io::Cursor, path::PathBuf};

#[derive(Debug)]
struct Yay0Hdr {
    magic: [u8; 4],
    len: usize,
    link_offset: usize,
    data_offset: usize,
}

impl Yay0Hdr {
    fn from_arr(arr: [u8; 16]) -> Self {
        Self {
            magic: arr[0..4].try_into().unwrap(),
            len: u32::from_be_bytes(arr[4..8].try_into().unwrap()) as usize,
            link_offset: u32::from_be_bytes(arr[8..12].try_into().unwrap()) as usize,
            data_offset: u32::from_be_bytes(arr[12..16].try_into().unwrap()) as usize,
        }
    }
}

fn decompress_count(ptr: &[u8]) -> Result<(Vec<u8>, usize)> {
    let hdr = Yay0Hdr::from_arr(ptr[0..16].try_into().unwrap());
    // println!("{:?}", hdr);
    if &hdr.magic != b"Yay0" {
        bail!("Bad Yay0 magic bites:\n{:?}", hdr)
    }

    let mut out = Vec::with_capacity(hdr.len);
    let mut ctrl = BitReader::endian(Cursor::new(&ptr[16..]), BigEndian);
    let mut links = ByteReader::endian(Cursor::new(&ptr[hdr.link_offset..]), BigEndian);
    let mut links_read = 0;
    let mut data = ByteReader::endian(Cursor::new(&ptr[hdr.data_offset..]), BigEndian);
    let mut data_read = 0;

    while out.len() < hdr.len {
        if ctrl.read_bit()? {
            out.push(data.read::<u8>()?);
            data_read += 1;
        } else {
            let op = links.read::<u16>()?;
            links_read += 2;
            let back = (op & 0xfff) as usize;
            let mut copyback = if op >> 12 != 0 {
                2 + (op >> 12)
            } else {
                data_read += 1;
                data.read::<u8>()? as u16 + 18
            };

            while copyback != 0 {
                out.push(out[out.len() - back - 1]);
                copyback -= 1;
            }
        }
    }

    let compressed_size = match hdr.link_offset.cmp(&hdr.data_offset) {
        Ordering::Less => hdr.data_offset + data_read,
        Ordering::Greater => hdr.link_offset + links_read,
        Ordering::Equal => unreachable!(),
    };

    Ok((out, compressed_size))
}

pub fn find_yay0_files(p: PathBuf) -> Result<()> {
    let rom = std::fs::read(p).context("reading ROM")?;
    let re = Regex::new("Yay0").unwrap();

    for mat in re.find_iter(&rom) {
        let start = mat.start();
        let data = &rom[start..];
        let (decomp, compsize) = decompress_count(data).context("decompressing yay0")?;
        let is_szp = &rom[start - 0x18..start - 0x10] == b"PERS-SZP";
        let frag = if &decomp[8..16] == b"FRAGMENT" {
            let hdr: [u8; 0x20] = decomp[..0x20].try_into().unwrap();
            let num = crate::frags::Fragment::try_from(hdr)?.number();
            Cow::from(format!("Frag #{num} "))
        } else {
            Cow::from("")
        };
        let possible_next = if start >= 0x7C0000 && start < 0x8CC400 {
            align4(start + compsize)
        } else if start >= 0x16F2814 && start < 0x17DC340 {
            if (start + compsize) & 4 == 0 {
                start + compsize 
            } else {
                align16(start + compsize) + 4
            }
        } else {
            align16(start + compsize)
        };
        println!(
            "{:#X} - {}Yay0 {}-> {:#X} {:#X} [{:5x} unpack to {:5x}]",
            start - if is_szp { 0x18 } else { 0 },
            if is_szp { "SZP " } else { "" },
            frag,
            start + compsize,
            possible_next,
            compsize,
            decomp.len()
        );
    }

    Ok(())
}

const fn align(by: usize, x: usize) -> usize {
    return (x + (by - 1)) & !(by - 1)
}

const fn align16(x: usize) -> usize {
    return align(16, x)
}

const fn align4(x: usize) -> usize {
    return align(4, x)
}
