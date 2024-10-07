pub fn base16_encode_text(input: &str) -> Result<String, String> {
    // Base16 character set, which contains 16 printable characters for encoding.
    const BASE16_CHARS: &[u8] = b"0123456789ABCDEF";

    // Converts the input string to a byte sequence.
    let input_bytes = input.as_bytes();
    // An output vector of sufficient size is pre-allocated to store the encoded bytes.
    // Since each byte becomes two hex characters, the capacity is input_bytes.len() * 2.
    let mut output = Vec::with_capacity(input_bytes.len() * 2);

    // Traversing the sequence of input bytes.
    for &byte in input_bytes.iter() {
        // Each byte is divided into two groups of 4 bits and encoded.
        let nybble1 = (byte >> 4) & 0x0F;
        let nybble2 = byte & 0x0F;

        // Add the encoded characters to the output vector.
        output.push(BASE16_CHARS[nybble1 as usize]);
        output.push(BASE16_CHARS[nybble2 as usize]);
    }

    // Convert the output vector to a UTF-8 string.
    // Since we are only using ASCII characters, this conversion is guaranteed to succeed.
    // However, we still use `from_utf8` to maintain a consistent API with potential future extensions.
    String::from_utf8(output).map_err(|e| format!("Failed to convert to UTF-8: {}", e))
}

pub fn base64_encode_text(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("A value is required but not provided".to_string());
    };
    // Base64 character set, which contains 64 printable characters for encoding.
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Converts the input string to a byte sequence.
    let input_bytes = input.as_bytes();
    // An output vector of sufficient size is pre-allocated to store the encoded bytes.
    let mut output = Vec::with_capacity((input_bytes.len() + 2) / 3 * 4);

    // Traversing the sequence of input bytes in groups of 3 bytes and
    // extracting bytes based on the number of bytes currently grouped.
    for chunk in input_bytes.chunks(3) {
        let (byte0, byte1, byte2) = match chunk.len() {
            3 => (chunk[0], chunk[1], chunk[2]),
            2 => (chunk[0], chunk[1], 0),
            1 => (chunk[0], 0, 0),
            _ => return Err("Invalid chunk length".to_string()),
        };

        // Each byte is divided into groups of 6 bits and encoded.
        let sextet1 = (byte0 >> 2) & 0x3F;
        let sextet2 = ((byte0 & 0x03) << 4) | ((byte1 >> 4) & 0x0F);
        let sextet3 = ((byte1 & 0x0F) << 2) | ((byte2 >> 6) & 0x03);
        let sextet4 = byte2 & 0x3F;

        // Add the encoded bytes to the output vector and
        // padding with '=' if the byte group is less than 3 in length.
        output.push(BASE64_CHARS[sextet1 as usize]);
        output.push(BASE64_CHARS[sextet2 as usize]);
        output.push(if chunk.len() > 1 {
            BASE64_CHARS[sextet3 as usize]
        } else {
            b'='
        });
        output.push(if chunk.len() == 3 {
            BASE64_CHARS[sextet4 as usize]
        } else {
            b'='
        });
    }

    // Convert the output vector to a UTF-8 string and handle any conversion errors.
    String::from_utf8(output).map_err(|e| format!("Failed to convert to UTF-8: {}", e))
}

pub fn base_encode_text(input: &str) -> Result<[String; 2], String> {
    let base16_encoded = base16_encode_text(input);
    let base64_encoded = base64_encode_text(input);

    match (base16_encoded, base64_encoded) {
        (Ok(base16), Ok(base64)) => Ok([base16, base64]),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}
