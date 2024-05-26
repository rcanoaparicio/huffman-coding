use std::collections::HashMap;

pub fn decode_data(encoded_data: &Vec<u8>) -> Vec<u8> {
    let chars_number = encoded_data[0];
    let mut result = Vec::<u8>::new();
    let mut char_idx = 1;
    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();
    for _ in 0..chars_number {
        let c = encoded_data[char_idx];
        let n = encoded_data[char_idx + 1] as usize;
        let mut encoding: Vec<u8> = Vec::new();
        for j in (char_idx + 2)..(char_idx + 2 + n) {
            encoding.push(encoded_data[j]);
        }
        dictionary.insert(encoding, c);
        char_idx += n + 2;
    }

    let mut next_char_encoding: Vec<u8> = Vec::new();
    let mut mask = 128;
    for i in char_idx..encoded_data.len() {
        let c = encoded_data[i];
        while mask > 0 {
            let next_value = (c & mask) != 0;
            next_char_encoding.push(next_value as u8);
            match dictionary.get(&next_char_encoding) {
                Some(decoded_char) => {
                    result.push(*decoded_char);
                    next_char_encoding.clear();
                }
                None => {}
            }
            mask /= 2;
        }
        mask = 128;
    }

    result
}
