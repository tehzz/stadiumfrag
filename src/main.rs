mod frags;
mod jpeg;
mod szp;
mod yay0;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Helper for figuring out ROM map for POKESTADIUM"
)]
enum Cli {
    /// print info about relocatable fragments
    Fragments {
        /// path to ROM
        rom: PathBuf,
        /// print info about sections in fragments
        #[arg(short, long)]
        sections: bool,
    },
    /// Yay0 file info and commands (maybe)
    Yay0 {
        /// path to ROM
        rom: PathBuf,
    },
    // PRESJPEG info
    Jpeg {
        rom: PathBuf,
    },
    // PRES-SZP file info?
    Szp {
        rom: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Fragments { rom, sections } => frags::find_fragments(rom, sections),
        Cli::Yay0 { rom } => yay0::find_yay0_files(rom),
        Cli::Jpeg { rom } => jpeg::find_presjpeg(rom),
        Cli::Szp { rom } => szp::find_pers_szp(rom),
    }
}
