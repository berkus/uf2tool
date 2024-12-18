use anyhow::{anyhow, Error};
use argh::FromArgs;
use culpa::throws;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use uftwo::Block;

/// Parse blocks from an UF2 file
#[derive(FromArgs)]
struct Args {
    /// input uf2 file
    #[argh(positional)]
    file: PathBuf,
}

// reqwest and cache https://github.com/microsoft/uf2/raw/refs/heads/master/utils/uf2families.json

#[throws]
fn main() {
    let args: Args = argh::from_env();

    let file = File::open(args.file)?;
    let mut read = BufReader::with_capacity(512, file);

    while let Ok(bytes) = read.fill_buf() {
        if bytes.is_empty() {
            break;
        }
        let block = Block::from_bytes(bytes).map_err(|e| anyhow!("Block parse failed {:?}", e))?;
        println!("{:08x}", block.board_family_id_or_file_size);
        let len = bytes.len();
        read.consume(len);
    }
}
