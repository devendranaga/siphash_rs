#[path = "./siphash.rs"]
mod siphash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut key : [u8; 16] = [
            0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15
        ];
        let mut input : [u8; 16] = [
            0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15
        ];
        let mut out : [u8; 16] = [0; 16];

        siphash::siphash(&mut input, 16,
                         &mut key, &mut out, 16);

        println!("{:x?}", out);
    }
}
