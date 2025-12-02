use std::intrinsics::{unchecked_add, unchecked_mul, unchecked_sub};

const BASE_LUT: [u64; 11] = [0, 0, 11, 0, 101, 0, 1001, 0, 10001, 0, 100001];
const END_LUT: [u64; 11] = [
    0,
    0,
    100,
    0,
    10000,
    0,
    1000000,
    0,
    100000000,
    0,
    10000000000,
];
const START_LUT: [u64; 11] = [0, 0, 10, 0, 1000, 0, 100000, 0, 10000000, 0, 1000000000];

const PRECOMPUTED_M: [u64; 10] = {
    let mut m = [0u64; 10];
    let mut i = 0;
    while i < 10 {
        m[i] = if BASE_LUT[i] > 1 {
            compute_m_u32(BASE_LUT[i] as u32)
        } else {
            0
        };
        i += 1;
    }
    m
};
const PRECOMPUTED_M_LEN_10: u128 = compute_m_u64(BASE_LUT[10]);

#[inline]
const fn compute_m_u64(d: u64) -> u128 {
    (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF / d as u128) + 1
}

#[inline]
const fn compute_m_u32(d: u32) -> u64 {
    (0xFFFFFFFFFFFFFFFF / d as u64) + 1
}

#[inline]
const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
    unsafe {
        let mut bottom_half = unchecked_mul(lowbits & 0xFFFFFFFFFFFFFFFF, d as u128);
        bottom_half = bottom_half.unchecked_shr(64);
        let top_half = unchecked_mul(lowbits.unchecked_shr(64), d as u128);
        let both_halves = bottom_half.unchecked_add(top_half);
        (both_halves.unchecked_shr(64)) as u64
    }
}

#[inline]
const fn fastdiv_u64(a: u64, m: u128) -> u64 {
    mul128_u64(m, a)
}

#[inline]
const fn mul128_u32(lowbits: u64, d: u32) -> u64 {
    (lowbits as u128 * d as u128 >> 64) as u64
}

#[inline]
const fn fastdiv_u32(a: u32, m: u64) -> u32 {
    mul128_u32(m, a) as u32
}

#[inline(always)]
fn get_multiple_sum(start: u64, end: u64, base_idx: usize) -> u64 {
    unsafe {
        let (base, first_multiple, last_multiple) = if base_idx == 10 {
            let base = 10000000000;
            let m = PRECOMPUTED_M_LEN_10;
            (base, fastdiv_u64(start + base - 1, m), fastdiv_u64(end, m))
        } else {
            let base = *BASE_LUT.get_unchecked(base_idx);
            let m = *PRECOMPUTED_M.get_unchecked(base_idx);
            (
                base,
                fastdiv_u32((start + base - 1) as u32, m) as u64,
                fastdiv_u32(end as u32, m) as u64,
            )
        };

        base.unchecked_mul(
            (unchecked_mul(
                unchecked_add(first_multiple, last_multiple),
                unchecked_sub(last_multiple, first_multiple) + 1,
            )) / 2,
        )
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut solution: u64 = 0;
        let mut idx = 0;
        let input = input.as_bytes();
        while idx < input.len() {
            let start_start = idx;
            let mut start: u64 = 0;
            while *input.get_unchecked(idx) != b'-' {
                start = unchecked_add(
                    start.unchecked_mul(10_u64),
                    (input.get_unchecked(idx).unchecked_sub(b'0')) as u64,
                );
                idx = idx.unchecked_add(1);
            }
            let start_digits = idx.unchecked_sub(start_start);
            idx = idx.unchecked_add(1);

            let end_start = idx;
            let mut end: u64 = 0;
            // both , and \n are smaller than b'0'
            while *input.get_unchecked(idx) >= b'0' {
                end = unchecked_add(
                    end.unchecked_mul(10_u64),
                    (input.get_unchecked(idx).unchecked_sub(b'0')) as u64,
                );
                idx = idx.unchecked_add(1);
            }
            let end_digits = idx.unchecked_sub(end_start);
            idx = idx.unchecked_add(1);

            if !(start_digits & 1 == 1 && start_digits & 1 == end_digits & 1) {
                if end_digits == start_digits {
                    solution = solution.unchecked_add(get_multiple_sum(start, end, start_digits));
                } else if start_digits & 1 == 0 {
                    solution = solution.unchecked_add(get_multiple_sum(
                        start,
                        *END_LUT.get_unchecked(start_digits),
                        start_digits,
                    ));
                } else {
                    solution = solution.unchecked_add(get_multiple_sum(
                        *START_LUT.get_unchecked(start_digits + 1),
                        end,
                        start_digits + 1,
                    ));
                }
            }
        }
        solution
    }
}
