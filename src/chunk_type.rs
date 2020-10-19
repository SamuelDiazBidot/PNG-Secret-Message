use std::str::FromStr;
use std::{ error, fmt, str };
use std::convert::{ TryInto, TryFrom };

#[derive(Debug)]
pub struct ChunkType {
    data : [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.data
    }

    pub fn is_valid(&self) -> bool {
        let is_alpha = self.bytes().iter().all(|&byte| (byte as char).is_ascii_alphabetic());
        is_alpha && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        let mask: u8 = 0b00100000;
        let result = self.data[0] & mask;
        result != mask
    }

    pub fn is_public(&self) -> bool {
        let mask: u8 = 0b00100000;
        let result = self.data[1] & mask;
        result != mask
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let mask: u8 = 0b00100000;
        let result = self.data[2] & mask;
        result != mask
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let mask: u8 = 0b00100000;
        let result = self.data[3] & mask;
        result == mask
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = str::from_utf8(&self.data).map_err(|_| fmt::Error)?;
        write!(f,"{}", string)
    }
}

impl FromStr for ChunkType {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_bytes= s.as_bytes().try_into()?;
        let chunk = ChunkType { data: str_bytes };

        let is_alpha = chunk.bytes().iter().all(|&byte| (byte as char).is_ascii_alphabetic());
        if !is_alpha {
            return Err(Box::new(InvalidChunkTypeError));
        }

        Ok(chunk)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { data: value })
    }
}

#[derive(Debug)]
pub struct InvalidChunkTypeError;

impl error::Error for InvalidChunkTypeError {}

impl fmt::Display for InvalidChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid chunk type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
