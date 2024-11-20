use std::{error::Error, fmt::Display, fs::{read, File}, io::{self, Write}, str::FromStr};

use crate::{args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs}, chunk::Chunk, chunk_type::{ChunkType, ChunkTypeError}, png::{Png, PngError}};

#[derive(Debug)]
pub enum CommandProcessError {
    FileError(io::Error),
    IllegalChunkType(ChunkTypeError),
    PngParseError(PngError),
}

impl Display for CommandProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandProcessError::FileError(error) => write!(f, "{}", error),
            CommandProcessError::IllegalChunkType(chunk_type_error) => write!(f, "{}", chunk_type_error),
            CommandProcessError::PngParseError(png_error) => write!(f, "{}", png_error),
        }
    }
}

impl Error for CommandProcessError {}

impl From<io::Error> for CommandProcessError {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<ChunkTypeError> for CommandProcessError {
    fn from(value: ChunkTypeError) -> Self {
        Self::IllegalChunkType(value)
    }
}

impl From<PngError> for CommandProcessError {
    fn from(value: PngError) -> Self {
        Self::PngParseError(value)
    }
}

pub fn encode(args: EncodeArgs) -> Result<(), CommandProcessError> {
    let bytes = read(args.file_path.clone())?;
    let mut png = Png::try_from(bytes.as_ref())?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;

    let new_chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(new_chunk);

    let file_path = if let Some(p) = args.output_file { p } else { args.file_path };
    let mut file = File::create(file_path)?;
    file.write_all(&png.as_bytes())?;

    println!("Encode successfully");
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<(), CommandProcessError> {
    let bytes = read(args.file_path.clone())?;
    let png = Png::try_from(bytes.as_ref())?;

    let chunks = png.chunk_by_type(&args.chunk_type);
    if !chunks.is_empty() {
        for c in chunks {
            let s = c.data_as_string()
                .unwrap_or_else(|_| "/// Invalid UTF-8 String ///".to_string());
    
            println!("{}", s);
        }
    } else {
        println!("This file has no such chunks");
    }

    println!("Decode successfully");
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<(), CommandProcessError> {
    let bytes = read(args.file_path.clone())?;
    let mut png = Png::try_from(bytes.as_ref())?;

    if let Some(removed_chunk) = png.remove_first_chunk(&args.chunk_type) {
        println!("Removed chunk:\n{}", removed_chunk);
    } else {
        println!("This file has no such chunks");
    }

    let mut file = File::create(args.file_path)?;
    file.write_all(&png.as_bytes())?;

    println!("Removed successfully");
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<(), CommandProcessError> {
    let bytes = read(args.file_path.clone())?;
    let png = Png::try_from(bytes.as_ref())?;

    println!("{}", png);

    println!("End of printing");
    Ok(())
}