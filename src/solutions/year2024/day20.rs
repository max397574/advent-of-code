const NEIGHBOUR_OFFSETS: [isize; 4] = [-142, 142, 1, -1];

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    unsafe {
        let input = input.as_bytes();
        let mut start = 0;
        let mut end = 0;
        for (i, &byte) in input.iter().enumerate() {
            if byte == b'S' {
                start = i;
            }
            if byte == b'E' {
                end = i;
            }
        }
        let mut path = [false; 141 * 142];
        let mut costs = [0; 141 * 142];
        let mut cost = 0;
        let mut pos = start;
        loop {
            *path.get_unchecked_mut(pos) = true;
            *costs.get_unchecked_mut(pos) = cost;
            cost += 1;
            if pos == end {
                break;
            }

            for offset in NEIGHBOUR_OFFSETS {
                let new_pos = pos.wrapping_add_signed(offset);
                if new_pos < 141 * 142 - 1
                    && *input.get_unchecked(new_pos) != b'\n'
                    && *input.get_unchecked(new_pos) != b'#'
                    && !path.get_unchecked(new_pos)
                {
                    pos = new_pos;
                    break;
                }
            }
        }

        // iterate through all positions and figure out if a cheat could have saved time
        const CHEAT_POSITIONS: [isize; 8] = [-284, 284, 2, -2, 143, -143, 141, -141];

        for x in 0..141 {
            for y in 0..141 {
                let pos = x + y * 142;
                if path[pos] {
                    for offset in CHEAT_POSITIONS {
                        let new_pos = pos.wrapping_add_signed(offset);
                        if *path.get_unchecked(new_pos)
                            && costs.get_unchecked(new_pos) + 2 < *costs.get_unchecked(pos)
                            && costs.get_unchecked(pos) - costs.get_unchecked(new_pos) - 2 >= 100
                        {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    unsafe {
        let input = input.as_bytes();
        let mut start = 0;
        let mut end = 0;
        for (i, &byte) in input.iter().enumerate() {
            if byte == b'S' {
                start = i;
            }
            if byte == b'E' {
                end = i;
            }
        }
        let mut path = [false; 141 * 142];
        let mut costs = [0; 141 * 142];

        let mut cost = 0;
        let mut pos = start;
        loop {
            *path.get_unchecked_mut(pos) = true;
            *costs.get_unchecked_mut(pos) = cost;
            cost += 1;
            if pos == end {
                break;
            }

            for offset in NEIGHBOUR_OFFSETS {
                let new_pos = pos.wrapping_add_signed(offset);
                if new_pos < 141 * 142 - 1
                    && *input.get_unchecked(new_pos) != b'\n'
                    && *input.get_unchecked(new_pos) != b'#'
                    && !path.get_unchecked(new_pos)
                {
                    pos = new_pos;
                    break;
                }
            }
        }

        for x in 1..140 {
            for y in 1..140 {
                let pos = x + y * 142;
                if *path.get_unchecked(pos) && *costs.get_unchecked(pos) >= 100 {
                    for dx in -20_isize..=20 {
                        let distance_left = 20 - dx.abs();
                        for dy in -distance_left..=distance_left {
                            let (new_x, new_y) =
                                (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                            let new_pos = new_x + 142 * new_y;
                            if new_x < 141
                                && new_y < 141
                                && new_pos < 141 * 142 - 1
                                && *path.get_unchecked(new_pos)
                            {
                                let cheat_cost = dx.abs() + dy.abs();
                                if costs.get_unchecked(new_pos) + cheat_cost
                                    < *costs.get_unchecked(pos)
                                    && costs.get_unchecked(pos)
                                        - costs.get_unchecked(new_pos)
                                        - cheat_cost
                                        >= 100
                                {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
