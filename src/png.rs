use std::{error::Error, fmt::Display, str::FromStr};

use crate::{chunk::{Chunk, ChunkError}, chunk_type::ChunkType};

#[derive(Debug)]
pub enum PngErr {
    InvalidChunk(ChunkError),
    IllegalSignature,
}

impl Display for PngErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PngErr::InvalidChunk(chunk_error) => write!(f, "{}", chunk_error),
            PngErr::IllegalSignature => write!(f, "Illegal file signature: is this a png file?"),
        }
    }
}

impl Error for PngErr {}

pub struct Png {
    signature: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { signature: Self::STANDARD_HEADER, chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Option<Chunk> {
        if let Ok(chunk_type) = ChunkType::from_str(chunk_type) {
            let pos = self.chunks.iter().position(|c| *c.chunk_type() == chunk_type);
            pos.map(|p| self.chunks.remove(p))
        } else {
            None
        }
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.signature
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<Vec<&Chunk>> {
        if let Ok(chunk_type) = ChunkType::from_str(chunk_type) {
            Some(self.chunks.iter().filter(|c| *c.chunk_type() == chunk_type).collect())
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.signature.iter()
            .copied()
            .chain(self.chunks.iter().flat_map(|c| c.as_bytes().into_iter()))
            .collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = PngErr;
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 8 || Self::STANDARD_HEADER != &value[0..8] {
            return Err(Self::Error::IllegalSignature);
        }

        let mut pos: usize = 8;
        let mut buffer = [0u8; 4];
        let mut chunks = Vec::new();
        loop {
            buffer.copy_from_slice(&value[pos..pos + 4]);
            let length = u32::from_be_bytes(buffer) as usize;

            if pos + length + 12 > value.len() {
                return Err(Self::Error::InvalidChunk(ChunkError::IllegalLength));
            }
            
            let chunk = Chunk::try_from(&value[pos..pos + 12 + length])
                .map_err(|e| Self::Error::InvalidChunk(e))?;
            chunks.push(chunk);

            pos += 12 + length;
            if pos == value.len() {
                break;
            }
        }

        Ok(Self::from_chunks(chunks))
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature: {:?}\n\nChunks:\n{:?}", self.signature, self.chunks)
    }
}