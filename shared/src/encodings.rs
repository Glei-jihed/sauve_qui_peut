pub fn encode_b64(data: &[u8]) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/";
    let mut encoded = String::new();

    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        let valid = if i + 2 < data.len() { 3 } else if i + 1 < data.len() { 2 } else { 1 };

        let c0 = ((triple >> 18) & 0x3F) as u8;
        let c1 = ((triple >> 12) & 0x3F) as u8;
        let c2 = ((triple >> 6) & 0x3F) as u8;
        let c3 = (triple & 0x3F) as u8;

        encoded.push(alphabet[c0 as usize] as char);
        encoded.push(alphabet[c1 as usize] as char);
        if valid > 1 {
            encoded.push(alphabet[c2 as usize] as char);
        }
        if valid > 2 {
            encoded.push(alphabet[c3 as usize] as char);
        }
        i += 3;
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_b64() {
        assert_eq!(encode_b64(&[0]), "aa");
        assert_eq!(encode_b64(&[25]), "gq");
        assert_eq!(encode_b64(&[26]), "gG");
        assert_eq!(encode_b64(&[51]), "mW");
        assert_eq!(encode_b64(&[52]), "na");
        assert_eq!(encode_b64(&[61]), "pq");
        assert_eq!(encode_b64(&[62]), "pG");
        assert_eq!(encode_b64(&[63]), "pW");
        assert_eq!(encode_b64(b"Hello, World!"), "sgvSBg8SifDVCMXKiq");
    }
}

pub fn encode_labyrinth(nx: u16, ny: u16, horizontal: &[u8], vertical: &[u8]) -> String {
    let mut data = Vec::new();
    data.extend(&nx.to_le_bytes());
    data.extend(&ny.to_le_bytes());
    data.extend(horizontal);
    data.extend(vertical);
    encode_b64(&data)
}

pub fn encode_radar(horiz: &[u8; 3], vert: &[u8; 3], cells: &[u8; 5]) -> String {
    let mut data = Vec::new();
    data.extend_from_slice(horiz);
    data.extend_from_slice(vert);
    data.extend_from_slice(cells);
    encode_b64(&data)
}

#[cfg(test)]
mod radar_tests {
    use super::*;
    #[test]
    fn test_encode_labyrinth() {
        let nx = 10;
        let ny = 10;
        let horizontal = vec![0b11111111, 0b11000000];
        let vertical = vec![0b10101010, 0b01010101];
        let encoded = encode_labyrinth(nx, ny, &horizontal, &vertical);
        assert!(!encoded.is_empty());
    }
    #[test]
    fn test_encode_radar() {
        let horiz = [0x12, 0x34, 0x56];
        let vert = [0x78, 0x9A, 0xBC];
        let cells = [0xDE, 0xF0, 0x12, 0x34, 0x56];
        let encoded = encode_radar(&horiz, &vert, &cells);
        assert!(!encoded.is_empty());
    }
}
