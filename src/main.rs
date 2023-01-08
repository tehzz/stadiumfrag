mod frags;
mod jpeg;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Fragments { rom } => frags::find_fragments(rom),
        Cli::Yay0 { rom } => yay0::find_yay0_files(rom),
        Cli::Jpeg { rom } => jpeg::find_presjpeg(rom),
    }
}
