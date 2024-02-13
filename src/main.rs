use std::{
    fs::File,
    io::{copy, BufReader, Error},
    path::PathBuf,
};

use sha2::{Digest, Sha256};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "sha256sum",
    about = "Generate checksum for files with the sha256 algorithm"
)]
pub struct CommandLine {
    /// File to be hashed
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}

fn main() -> Result<(), Error> {
    let cmd = CommandLine::from_args();
    let input = File::open(cmd.file)?;
    let mut reader = BufReader::new(input);

    let mut hasher = Sha256::new();
    copy(&mut reader, &mut hasher)?;
    let digest = hasher.finalize();

    println!("{digest:x}");

    Ok(())
}
