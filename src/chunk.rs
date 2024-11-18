use std::{fmt::Display, string::FromUtf8Error};

use crc::Crc;

use crate::chunk_type::{ChunkType, ChunkTypeError};

#[derive(Debug)]
pub enum ChunkError {
    IllegalLength,
    IllegalChunkType(ChunkTypeError),
    IncorrectChecksum,

}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::IllegalLength => write!(f, "Error while constructing: field 'length' must greater than 12 and fits data"),
            ChunkError::IllegalChunkType(chunk_type_error) => write!(f, "{}", chunk_type_error),
            ChunkError::IncorrectChecksum => write!(f, "Incorrect checksum: is your png broken?"),
        }
    }
}

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err(Self::Error::IllegalLength);
        }

        let mut buffer = [0u8; 4];
        buffer.copy_from_slice(&value[0..4]);
        let length = u32::from_be_bytes(buffer);
        let length_usize = length as usize;

        if value.len() != 12 + length_usize {
            return Err(Self::Error::IllegalLength);
        }

        buffer.copy_from_slice(&value[4..8]);
        let chunk_type = ChunkType::try_from(buffer)
            .map_err(|e| Self::Error::IllegalChunkType(e))?;

        let data= value[8..length_usize + 8].to_vec();
        buffer.copy_from_slice(&value[8 + length_usize..12 + length_usize]);
        let checksum = u32::from_be_bytes(buffer);

        if checksum != Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&value[4..8 + length_usize]) {
            return Err(Self::Error::IncorrectChecksum);
        }

        Ok(Self { length, chunk_type, data, crc: checksum })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data_as_string().unwrap_or_else(|_| String::new());

        write!(f,
            "Chunk information:\n\tlength: {0},\n\ttype: {1},\n\tdata: {2},\n\tcrc: {3}",
            self.length,
            self.chunk_type,
            data,
            self.crc
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        let bytes = [&chunk_type.bytes()[..], &data].concat();
        let checksum = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&bytes);
        Self { length, chunk_type, data, crc: checksum }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [&self.length.to_be_bytes()[..], &self.chunk_type.bytes(), &self.data, &self.crc.to_be_bytes()].concat()
    }
}