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
