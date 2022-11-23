const CROUNDS : u32 = 2;
const DROUNDS : u32 = 4;

pub fn rotl(val : u64, bits : u64) -> u64 {
    return (val << bits) | (val >> (64 - bits));
}

pub fn u64to8_le(buf : &mut[u8], val : u64) {
    buf[0] = ((val & 0xFF00000000000000) >> 56) as u8;
    buf[1] = ((val & 0x00FF000000000000) >> 48) as u8;
    buf[2] = ((val & 0x0000FF0000000000) >> 40) as u8;
    buf[3] = ((val & 0x000000FF00000000) >> 32) as u8;
    buf[4] = ((val & 0x00000000FF000000) >> 24) as u8;
    buf[5] = ((val & 0x0000000000FF0000) >> 16) as u8;
    buf[6] = ((val & 0x000000000000FF00) >> 8) as u8;
    buf[7] = (val & 0x00000000000000FF) as u8;
}

pub fn u8to64_le(buf: &mut[u8]) -> u64 {
    let val = buf[0] as u64 |
              ((buf[1] as u64) << 8) |
              ((buf[2] as u64) << 16) |
              ((buf[3] as u64) << 24) |
              ((buf[4] as u64) << 32) |
              ((buf[5] as u64) << 40) |
              ((buf[6] as u64) << 48) |
              ((buf[7] as u64) << 56);
    val
}

pub fn sipround(v0 : &mut u64, v1 : &mut u64,
                v2 : &mut u64, v3 : &mut u64) {
    *v0 += *v1;
    *v1 = rotl(*v1, 13);
    *v1 ^= *v0;
    *v0 = rotl(*v0, 32);
    *v2 += *v3;
    *v3 = rotl(*v3, 16);
    *v3 ^= *v2;
    *v0 += *v3;
    *v3 = rotl(*v3, 21);
    *v3 ^= *v0;
    *v2 += *v1;
    *v1 = rotl(*v1, 17);
    *v1 ^= *v2;
    *v2 = rotl(*v2, 32);
}

fn push_b(ni : &mut [u8], off : usize, b : &mut u64, left : usize) {
    for i in  0..left {
        println!("{} left {} left - 1 - i {} {}", ni[off - 1 - i], left, left - 1 - i, (left - i - 1) * 8);
        *b |= (ni[off - 1 - i] as u64) << ((left - i - 1) * 8);
    }
}

pub fn siphash(ni : &mut [u8], in_buf_len : usize,
               kk : &mut [u8], out_buf : &mut [u8],
               outbuf_len : usize) {

    assert!((outbuf_len == 8) || (outbuf_len == 16));

    let mut v0 : u64 = 0x736f6d6570736575;
    let mut v1 : u64 = 0x646f72616e646f6d;
    let mut v2 : u64 = 0x6c7967656e657261;
    let mut v3 : u64 = 0x7465646279746573;
    let k0 : u64 = u8to64_le(kk);
    let k1 : u64 = u8to64_le(&mut kk[8..]);
    let mut m : u64;
    let end : usize = in_buf_len - (in_buf_len % 8);
    let left : usize = in_buf_len & 0x7;
    let mut b : u64 = (in_buf_len as u64) << 56;

    v3 ^= k1;
    v2 ^= k0;
    v1 ^= k1;
    v0 ^= k0;

    if outbuf_len == 16 {
        v1 ^= 0xee;
    }

    let mut t : usize = 0;
    while t < end {
        m = u8to64_le(&mut ni[t..]);
        v3 ^= m;

        for _i in 0..CROUNDS {
            sipround(&mut v0, &mut v1, &mut v2, &mut v3);
        }

        v0 ^= m;
        t += 8;
    }

    push_b(&mut ni[t..], left, &mut b, left);

    v3 ^= b;

    for _i in 0..CROUNDS {
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    }

    v0 ^= b;

    if outbuf_len == 16 {
        v2 ^= 0xee;
    } else {
        v2 ^= 0xff;
    }

    for _i in 0..DROUNDS {
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    }

    b = v0 ^ v1 ^ v2 ^ v3;
    u64to8_le(out_buf, b);

    if outbuf_len == 16 {
        v1 ^= 0xdd;

        for _i in 0..DROUNDS {
            sipround(&mut v0, &mut v1, &mut v2, &mut v3);
        }

        b = v0 ^ v1 ^ v2 ^ v3;
        u64to8_le(&mut out_buf[8..], b);
    }
}

