use clap::{self, Parser};
use memchr::memchr;
use memmap2::Mmap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str::from_utf8_unchecked;

pub mod cli;
use cli::Args;

fn main() {
    let args = Args::parse();
    nexus2fa(args.nexus, args.fasta).expect("Error converting .nex to .fa!");
}

fn nexus2fa<T: AsRef<Path>>(nex: T, fa: T) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(nex)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let mut fa = BufWriter::new(File::create(fa)?);

    let start_block = find_substring(b"MATRIX", &mmap)
        .ok_or("ERROR: tried to find 'MATRIX' keyword but could not find any :(")?;
    let edge_start_block = memchr(b'\t', &mmap[start_block..])
        .ok_or("ERROR: failed trying to enter sequences block :(")?
        + start_block;
    let end_block = memchr(b';', &mmap[edge_start_block..])
        .ok_or("ERROR: check your .nex, this does not make sense...")?
        + edge_start_block;
    let block = &mmap[edge_start_block + 2..end_block];

    let lines = block.split(|&c| c == b'\n').collect::<Vec<_>>();
    for line in lines {
        if line.is_empty() || line == b"\t" {
            continue;
        }

        let line = unsafe { from_utf8_unchecked(line) };
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let name = parts[0];
        let sequence = parts[1];

        writeln!(fa, ">{}\n{}", name, sequence)?;
    }

    Ok(())
}

fn find_substring(needle: &[u8], haystack: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
