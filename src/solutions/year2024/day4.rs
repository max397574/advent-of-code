use bstr::ByteSlice;
use std::simd::{Simd, cmp::SimdPartialEq};

use crate::utils::grid::Grid;

fn get_bitvecs_p1(line: &[u8]) -> [[u64; 3]; 4] {
    let bit_vec1 = std::simd::u8x64::from_slice(&line[0..64]);
    let x_mask1 = bit_vec1.simd_eq(std::simd::u8x64::splat(b'X')).to_bitmask();
    let m_mask1 = bit_vec1.simd_eq(std::simd::u8x64::splat(b'M')).to_bitmask();
    let a_mask1 = bit_vec1.simd_eq(std::simd::u8x64::splat(b'A')).to_bitmask();
    let s_mask1 = bit_vec1.simd_eq(std::simd::u8x64::splat(b'S')).to_bitmask();
    let bit_vec2 = std::simd::u8x64::from_slice(&line[61..125]);
    let x_mask2 = bit_vec2.simd_eq(std::simd::u8x64::splat(b'X')).to_bitmask();
    let m_mask2 = bit_vec2.simd_eq(std::simd::u8x64::splat(b'M')).to_bitmask();
    let a_mask2 = bit_vec2.simd_eq(std::simd::u8x64::splat(b'A')).to_bitmask();
    let s_mask2 = bit_vec2.simd_eq(std::simd::u8x64::splat(b'S')).to_bitmask();
    let bit_vec3 = std::simd::u8x64::from_slice(&line[76..]);
    let x_mask3 = bit_vec3.simd_eq(std::simd::u8x64::splat(b'X')).to_bitmask() >> 46;
    let m_mask3 = bit_vec3.simd_eq(std::simd::u8x64::splat(b'M')).to_bitmask() >> 46;
    let a_mask3 = bit_vec3.simd_eq(std::simd::u8x64::splat(b'A')).to_bitmask() >> 46;
    let s_mask3 = bit_vec3.simd_eq(std::simd::u8x64::splat(b'S')).to_bitmask() >> 46;
    [
        [x_mask1, x_mask2, x_mask3],
        [m_mask1, m_mask2, m_mask3],
        [a_mask1, a_mask2, a_mask3],
        [s_mask1, s_mask2, s_mask3],
    ]
}

pub fn part1_speedy(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut count = 0;
    let mut x = [0, 0, 0];
    let mut x_prev = [0, 0, 0];
    let mut x_prev_prev = [0, 0, 0];
    let mut x_prev_prev_prev = [0, 0, 0];
    let mut m = [0, 0, 0];
    let mut m_prev = [0, 0, 0];
    let mut m_prev_prev = [0, 0, 0];
    let mut m_prev_prev_prev = [0, 0, 0];
    let mut a = [0, 0, 0];
    let mut a_prev = [0, 0, 0];
    let mut a_prev_prev = [0, 0, 0];
    let mut a_prev_prev_prev = [0, 0, 0];
    let mut s = [0, 0, 0];
    let mut s_prev = [0, 0, 0];
    let mut s_prev_prev = [0, 0, 0];
    let mut s_prev_prev_prev = [0, 0, 0];

    const OVERLAP_MASK: u64 = !(0b111) & !(0b111 << 61);

    input.lines().for_each(|line| {
        let vecs = get_bitvecs_p1(line);
        x_prev_prev_prev = x_prev_prev;
        x_prev_prev = x_prev;
        x_prev = x;
        x = vecs[0];
        m_prev_prev_prev = m_prev_prev;
        m_prev_prev = m_prev;
        m_prev = m;
        m = vecs[1];
        a_prev_prev_prev = a_prev_prev;
        a_prev_prev = a_prev;
        a_prev = a;
        a = vecs[2];
        s_prev_prev_prev = s_prev_prev;
        s_prev_prev = s_prev;
        s_prev = s;
        s = vecs[3];
        (0..3).for_each(|i| {
            // right
            count += (x[i] & (m[i] << 1) & (a[i] << 2) & (s[i] << 3)).count_ones();
            // left
            count += (x[i] & (m[i] >> 1) & (a[i] >> 2) & (s[i] >> 3)).count_ones();

            // up-right
            count += (x[i] & (m_prev[i] << 1) & (a_prev_prev[i] << 2) & (s_prev_prev_prev[i] << 3))
                .count_ones();
            // up-left
            count += (x[i] & (m_prev[i] >> 1) & (a_prev_prev[i] >> 2) & (s_prev_prev_prev[i] >> 3))
                .count_ones();

            // down-right
            count += (x_prev_prev_prev[i] & (m_prev_prev[i] << 1) & (a_prev[i] << 2) & (s[i] << 3))
                .count_ones();
            // down-left
            count += (x_prev_prev_prev[i] & (m_prev_prev[i] >> 1) & (a_prev[i] >> 2) & (s[i] >> 3))
                .count_ones();

            // set some bits to zero to get rid of overlapping stuff

            if i == 1 {
                // up
                count +=
                    (x[i] & (m_prev[i]) & (a_prev_prev[i]) & (s_prev_prev_prev[i] & OVERLAP_MASK))
                        .count_ones();
                // down
                count +=
                    (x_prev_prev_prev[i] & (m_prev_prev[i]) & (a_prev[i]) & (s[i] & OVERLAP_MASK))
                        .count_ones();
            } else {
                // up
                count +=
                    (x[i] & (m_prev[i]) & (a_prev_prev[i]) & (s_prev_prev_prev[i])).count_ones();
                // down
                count +=
                    (x_prev_prev_prev[i] & (m_prev_prev[i]) & (a_prev[i]) & (s[i])).count_ones();
            }
        });
    });
    count
}

pub fn part1_simple(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let mut count = 0;
    grid.iter().for_each(|((x, y), val)| {
        if *val == b'X' {
            let directions = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            let x = x as isize;
            let y = y as isize;
            for (dx, dy) in directions {
                if grid.get_i((x + dx * 3, y + dy * 3)) == Some(&b'S')
                    && grid.get_i((x + dx * 2, y + dy * 2)) == Some(&b'A')
                    && grid.get_i((x + dx, y + dy)) == Some(&b'M')
                {
                    count += 1;
                }
            }
        }
    });
    count
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    if input.len() < 140 {
        part1_simple(input)
    } else {
        part1_speedy(input)
    }
}

#[inline]
fn get_simds(input: &[u8], index: usize) -> [Simd<u8, 64>; 5] {
    let top_left = std::simd::u8x64::from_slice(&input[index * 64..]);
    let top_right = std::simd::u8x64::from_slice(&input[index * 64 + 2..]);
    let center = std::simd::u8x64::from_slice(&input[index * 64 + 141 + 1..]);
    let bottom_left = std::simd::u8x64::from_slice(&input[index * 64 + 141 * 2..]);
    let bottom_right = std::simd::u8x64::from_slice(&input[index * 64 + 141 * 2 + 2..]);
    [top_left, top_right, center, bottom_left, bottom_right]
}

pub fn part2_speedy(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut count = 0;

    // iterate over groups of 64
    (0..304).for_each(|i| {
        let simds = get_simds(input, i);
        // top left to bottom right
        let diagonal1 = (simds[0] ^ simds[4])
            .simd_eq(std::simd::u8x64::splat(b'M' ^ b'S'))
            .to_bitmask();
        // top right to bottom left
        let diagonal2 = (simds[1] ^ simds[3])
            .simd_eq(std::simd::u8x64::splat(b'M' ^ b'S'))
            .to_bitmask();
        let positions_center = simds[2].simd_eq(std::simd::u8x64::splat(b'A')).to_bitmask();
        count += (diagonal1 & diagonal2 & positions_center).count_ones();
    });
    count
}

pub fn part2_simple(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let mut count = 0;
    grid.iter().for_each(|((x, y), val)| {
        let x = x as isize;
        let y = y as isize;
        if *val == b'A'
            && ((grid.get_i((x - 1, y - 1)) == Some(&b'M')
                && grid.get_i((x + 1, y + 1)) == Some(&b'S')
                && grid.get_i((x - 1, y + 1)) == Some(&b'M')
                && grid.get_i((x + 1, y - 1)) == Some(&b'S'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'M'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'M')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'M'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'S')))
        {
            count += 1;
        }
    });
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    if input.len() < 140 {
        part2_simple(input)
    } else {
        part2_speedy(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("18"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("9"))
    }
}
