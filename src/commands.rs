use std::{fs, str::FromStr};
use std::io;
use std::io::prelude::*;
use std::convert::TryFrom;

use crate::args::{Decode, Encode, Remove, Print};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::Result;

pub fn encode(args: Encode) -> Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut f = fs::OpenOptions::new().write(true).read(true).create(true).open(args.file)?;

    f.read_to_end(&mut bytes)?;

    let mut png: Png = Png::try_from(bytes.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk: Chunk = Chunk::new(chunk_type, args.message.into_bytes());

    png.append_chunk(chunk);

    f.seek(io::SeekFrom::Start(0))?;
    f.write_all(png.as_bytes().as_slice())?;

    Ok(())
}

pub fn decode(args: Decode) -> Result<String> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let png: Png = Png::try_from(bytes)?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => return Ok(chunk.data_as_string()?),
        None => return Err("No such chunk type".into())
    }
}

pub fn remove(args: Remove) -> Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut f = fs::OpenOptions::new().write(true).read(true).open(args.file)?;

    f.read_to_end(&mut bytes)?;
    
    let mut png: Png = Png::try_from(bytes.as_slice())?;

    png.remove_chunk(&args.chunk_type)?;

    f.set_len(0)?;
    f.seek(io::SeekFrom::Start(0))?;
    f.write_all(png.as_bytes().as_slice())?;

    Ok(())
}

pub fn print_chunks(args: Print) -> Result<()> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let png: Png = Png::try_from(bytes)?;

    println!("{:?}", png.chunks());

    Ok(())
}