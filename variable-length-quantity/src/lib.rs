// I have ignored one test case, because it conflicts with the readme:
// https://github.com/exercism/xrust/issues/224

const MORE_BYTES_MARKER: u8 = 0x80;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(u32_to_bytes).collect()
}

pub fn u32_to_bytes(value: &u32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut tmp = *value;
    loop {
        let mut bits7 = (tmp as u8) & 0x7f;
        if result.len() > 0 {
            bits7 |= MORE_BYTES_MARKER; // mark all except first byte
        }
        tmp = tmp >> 7;
        result.push(bits7);
        if tmp == 0 {
            break;
        }
    }
    result.reverse();
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if !is_finished(&bytes[bytes.len() - 1]) {
        return Err("Incomplete byte sequence");
    }
    let mut result: Vec<u32> = Vec::new();
    let mut tmp: u32 = 0;
    let mut bytes_of_value_count = 0;
    for byte in bytes {
        bytes_of_value_count += 1;
        tmp += (byte & 0x7f) as u32;
        if is_finished(byte) {
            result.push(tmp);
            bytes_of_value_count = 0;
            tmp = 0;
        } else {
            if bytes_of_value_count > 3 {
                return Err("Overflow u32");
            }
            tmp = tmp << 7;
        }
    }
    Ok(result)
}

fn is_finished(byte: &u8) -> bool {
    byte & MORE_BYTES_MARKER == 0
}
