#[path = "./siphash.rs"]
pub mod siphash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_siphash_24() {
        let mut key : [u8; 16] = [
            0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15
        ];
        let mut input : [u8; 15] = [
            0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14
        ];
        let mut out : [u8; 8] = [0; 8];
        let expected : [u8; 8] = [
            0xa1, 0x29, 0xca, 0x61, 0x49, 0xbe, 0x45, 0xe5
        ];

        siphash::siphash(&mut input, 15,
                         &mut key, &mut out, 8);

        println!("{:x?}", out);

        for i in 0..8 {
            assert!(out[i] == expected[i]);
        }
    }
}
