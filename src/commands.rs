use std::{fs, str::FromStr};
use std::convert::TryFrom;

use crate::args::{Decode, Encode, Remove, Print};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

use std::path::Path;
use crate::{Result, Error};

pub fn encode(args: Encode) -> Result<()> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let mut png: Png = Png::try_from(bytes)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk: Chunk = Chunk::new(chunk_type, args.message.into_bytes());

    png.append_chunk(chunk);

    Ok(())
}

pub fn decode(args: Decode) -> Result<()> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let png: Png = Png::try_from(bytes)?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
                let data = chunk.data_as_string()?;
                println!("{}", data);       
        }, 
        None => return Err("No such chunk type".into())
    }

    Ok(())
}

pub fn remove(args: Remove) -> Result<()> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let mut png: Png = Png::try_from(bytes)?;

    png.remove_chunk(&args.chunk_type)?;

    Ok(())
}

pub fn print_chunks(args: Print) -> Result<()> {
    let bytes: &[u8] = &fs::read(args.file)?;
    let png: Png = Png::try_from(bytes)?;

    println!("{:?}", png.chunks());

    Ok(())

}