use std::fmt::{Display, Formatter};
use std::str::FromStr;

// see http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html 3.3 for definition of these bits
const ANCILLARY_BIT: usize = 0;
const PRIVATE_BIT: usize = 1;
const RESERVED_BIT: usize = 2;
const SAFE_TO_COPY_BIT: usize = 3;

#[derive(Debug)]
pub enum Error {
    TooManyBytes(String),
    MustBeAlphabetic(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let bytes = s.as_bytes();
            for byte in bytes {
                if !byte.is_ascii_alphabetic() {
                    return Err(Error::MustBeAlphabetic(format!(
                        "A ChunkType may only contain A-Z or a-z. Found {}",
                        std::str::from_utf8(std::slice::from_ref(byte)).unwrap()
                    )));
                }
            }
            ChunkType::try_from(<[u8; 4]>::try_from(s.as_bytes()).unwrap())
        } else {
            Err(Error::TooManyBytes(format!(
                "A ChunkType must be exactly 4 bytes, found {}",
                s.len()
            )))
        }
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        // self.bytes.is_ascii()
        for byte in self.bytes {
            if !byte.is_ascii_alphabetic() {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[ANCILLARY_BIT])
    }

    pub fn is_public(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[PRIVATE_BIT])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[RESERVED_BIT])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        u8::is_ascii_lowercase(&self.bytes[SAFE_TO_COPY_BIT])
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
