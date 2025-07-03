pub fn encode_hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, &'static str> {
    if !hex_str.len().is_multiple_of(2) {
        return Err("Hex string must have even length");
    }

    let mut result = Vec::new();
    for chunk in hex_str.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8")?;
        let byte = u8::from_str_radix(hex_byte, 16).map_err(|_| "Invalid hex character")?;
        result.push(byte);
    }

    Ok(result)
}

pub fn bytes_to_u32_le(bytes: &[u8]) -> u32 {
    let mut array = [0u8; 4];
    let len = bytes.len().min(4);
    array[..len].copy_from_slice(&bytes[..len]);
    u32::from_le_bytes(array)
}

pub fn u32_to_bytes_le(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}
