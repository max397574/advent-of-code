use num_traits::PrimInt;

pub trait ByteParsing {
    fn as_num<T: PrimInt>(&self) -> T;
}

impl ByteParsing for [u8] {
    fn as_num<T: PrimInt>(&self) -> T {
        let mut out = T::zero();
        for byte in self {
            out = out * T::from(10).unwrap() + T::from(byte - b'0').unwrap();
        }
        out
    }
}

#[inline]
pub fn swar_parsing(chunk: u64) -> u64 {
    let lower = chunk & 0x000f000f000f000f;
    let upper = (chunk & 0x0f000f000f000f00) >> 8;
    let chunk = 10 * upper + lower;

    let lower = chunk & 0x000000ff000000ff;
    let upper = (chunk & 0x00ff000000ff0000) >> 16;
    let chunk = 100 * upper + lower;

    let lower = chunk & 0x000000000000ffff;
    let upper = (chunk & 0x0000ffff00000000) >> 32;
    10000 * upper + lower
}
