use std::io::{self, Read};

pub fn write_uleb128(value: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let mut val = value;
    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if val == 0 {
            break;
        }
    }
    result
}

pub fn read_uleb128<R: Read>(reader: &mut R) -> io::Result<usize> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        let b = byte[0];
        result |= ((b & 0x7F) as usize) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

pub fn write_bstring(s: &str) -> Vec<u8> {
    if s.is_empty() {
        return vec![0];
    }
    let mut result = vec![11];
    result.extend(write_uleb128(s.len()));
    result.extend(s.as_bytes());
    result
}

pub fn read_bstring<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut indicator = [0u8; 1];
    reader.read_exact(&mut indicator)?;
    if indicator[0] != 11 {
        return Ok(String::new());
    }
    let length = read_uleb128(reader)?;
    let mut buffer = vec![0u8; length];
    reader.read_exact(&mut buffer)?;
    String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn write_i8(value: i8) -> [u8; 1] {
    value.to_le_bytes()
}

pub fn read_i8<R: Read>(reader: &mut R) -> io::Result<i8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(i8::from_le_bytes(buf))
}

pub fn write_u16(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

pub fn read_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

pub fn write_i32(value: i32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn read_i32<R: Read>(reader: &mut R) -> io::Result<i32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

pub fn write_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn read_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

pub fn write_i64(value: i64) -> [u8; 8] {
    value.to_le_bytes()
}

pub fn read_i64<R: Read>(reader: &mut R) -> io::Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

pub fn write_u64(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

pub fn write_bool(value: bool) -> [u8; 1] {
    [if value { 1 } else { 0 }]
}

pub fn read_bool<R: Read>(reader: &mut R) -> io::Result<bool> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0] > 0)
}

pub fn read_slice<R: Read>(reader: &mut R, length: i32) -> io::Result<Vec<u8>> {
    let mut buffer = vec![0u8; length as usize];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}
