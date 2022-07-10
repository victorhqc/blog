pub fn get_uuid_bytes(id: &Vec<u8>) -> [u8; 16] {
    let mut uuid: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for (index, byte) in id.iter().enumerate() {
        uuid[index] = byte.clone();
    }

    uuid
}
