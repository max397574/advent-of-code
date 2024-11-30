pub struct Block {
    cols: Vec<u32>,
    rows: Vec<u32>,
}

// Create bitmaps for rows and columns
fn parse(input: &str) -> Vec<Block> {
    input
        .split("\n\n")
        .map(|block| {
            let mut rows = vec![0];
            let mut cols = vec![0; block.find("\n").unwrap()];
            let mut col_idx = 0;
            let mut row_idx = 0;
            for char in block.chars() {
                if char == '\n' {
                    rows.push(0);
                    row_idx += 1;
                    col_idx = 0;
                    continue;
                }
                let mut val = 0;
                if char == '#' {
                    val = 1;
                }

                let col = cols.get_mut(col_idx).unwrap();
                *col <<= 1;
                *col += val;
                let row = rows.get_mut(row_idx).unwrap();
                *row <<= 1;
                *row += val;
                col_idx += 1;
            }
            Block { rows, cols }
        })
        .collect()
}

fn one_bit_different(x: u32, y: u32) -> bool {
    // Bit magic
    // Just works

    // use XOR to calcualte just the different bits
    let z = x ^ y;

    // check if there is just a single 1-bit
    // Same as checking if is power of 2
    z & (z - 1) == 0
}

fn try_reflect(block: &[u32], allow_smudge: bool) -> usize {
    let mut has_reflection = false;
    let mut has_smudge = !allow_smudge;
    let mut ref_idx = 0;
    // Iterate through all the possible symmetry-lines
    // check for each if it has a reflection by going outwards from it
    // and comparing the bitmaps
    for idx in 1..block.len() {
        has_reflection = true;
        has_smudge = !allow_smudge;
        ref_idx = idx;
        let mut lower = idx;
        let mut higher = idx - 1;

        while lower > 0 && higher < block.len() - 1 {
            lower -= 1;
            higher += 1;
            if block[lower] == block[higher] {
                continue;
            } else if !has_smudge && one_bit_different(block[lower], block[higher]) {
                has_smudge = true;
            } else {
                has_reflection = false;
                break;
            }
        }
        if has_reflection && (!allow_smudge || has_smudge) {
            break;
        }
    }
    if has_reflection && (!allow_smudge || has_smudge) {
        ref_idx
    } else {
        0
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let blocks = parse(input);
    blocks
        .iter()
        .map(|block| {
            let val = try_reflect(&block.rows, false);
            if val > 0 {
                val * 100
            } else {
                try_reflect(&block.cols, false)
            }
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let blocks = parse(input);
    blocks
        .iter()
        .map(|block| {
            let val = try_reflect(&block.rows, true);
            if val > 0 {
                val * 100
            } else {
                try_reflect(&block.cols, true)
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("405"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("400"))
    }
}
