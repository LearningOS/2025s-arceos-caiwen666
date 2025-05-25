use arceos_api::modules::axhal::misc::random;

pub fn hashmap_random_keys() -> (u64, u64) {
    let mut bytes = [0; 16];
    bytes.copy_from_slice(random().to_le_bytes().as_slice());
    let k1 = u64::from_ne_bytes(bytes[0..8].try_into().unwrap());
    let k2 = u64::from_ne_bytes(bytes[8..].try_into().unwrap());
    (k1, k2)
}
