use std::intrinsics::unchecked_sub;

static mut WIRES: [u8; 15700] = [3; 15700];
static mut GATES: [(usize, usize, u8, usize); 222] = [(0, 0, 0, 0); 222];
static mut GATE_OUTPUTS: [u8; 15700] = [3; 15700];

pub fn part1(input: &str) -> u64 {
    unsafe {
        let input = input.as_bytes();
        let mut input = input.as_ptr();

        WIRES.fill(u8::MAX);

        unsafe fn b26(val: [u8; 3]) -> usize {
            let [a, b, c] = [
                (unchecked_sub(val[0], b'a')) as usize,
                (unchecked_sub(val[1], b'a')) as usize,
                (unchecked_sub(val[2], b'a')) as usize,
            ];
            a * 26 * 26 + b * 26 + c
        }

        input = input.add("x00: ".len());

        for i in 0..45 {
            WIRES[15500 + i] = unchecked_sub(*input.offset(0), b'0');
            input = input.add("0\nx00: ".len());
        }

        for i in 0..45 {
            WIRES[15550 + i] = unchecked_sub(*input.offset(0), b'0');
            input = input.add("0\ny00: ".len());
        }

        input = input.sub("0\ny00: ".len());
        input = input.add("0\n\n".len());

        // gate: (in1,in2,op,out) use 0 as special op for already calculated

        // we interpret abc values as b26, can go up to 15466
        // so we use values above that for the xZZ, yZZ, zZZ (add 15500 plus another 50 to
        // distinguish)

        (0..222).for_each(|i| {
            let i1 = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                // is x,y,z so do as described above
                (unchecked_sub(*input.offset(1) as usize, b'0' as usize)) * 10
                    + unchecked_sub(*input.offset(2) as usize, b'0' as usize)
                    + 15500
                    + 50 * (unchecked_sub(*input.offset(0), b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            let op = *input.offset(4);
            input = input.add(7);
            if op != b'O' {
                input = input.add(1);
            }
            let i2 = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                (unchecked_sub(*input.offset(1) as usize, b'0' as usize)) * 10
                    + unchecked_sub(*input.offset(2) as usize, b'0' as usize)
                    + 15500
                    + 50 * (unchecked_sub(*input.offset(0), b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            input = input.add(7);
            let out = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                unchecked_sub(*input.offset(1) as usize, b'0' as usize) * 10
                    + unchecked_sub(*input.offset(2) as usize, b'0' as usize)
                    + 15500
                    + 50 * (unchecked_sub(*input.offset(0), b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            input = input.add(4);
            GATE_OUTPUTS[out] = i;
            GATES[i as usize] = (i1, i2, op, out);
        });

        // not sure how this works but it does
        // similar to what was done here https://fprijate.github.io/tlborm/mbe-min-debugging.html
        macro_rules! calc_unrolled {
    ($wire:expr, $($depth:tt)*) => {{
        calc_unrolled_inner!($wire, $($depth)*)
    }};
}

        macro_rules! calc_unrolled_inner {
    ($wire:expr, ) => {
        *WIRES.get_unchecked($wire)
    };
    ($wire:expr, $depth:tt $($rest:tt)*) => {{
        if *WIRES.get_unchecked($wire) != u8::MAX {
            *WIRES.get_unchecked($wire)
        } else {
            let gate = GATES.get_unchecked(GATE_OUTPUTS[$wire] as usize);
            let i1 = calc_unrolled_inner!(gate.0, $($rest)*);
            let i2 = calc_unrolled_inner!(gate.1, $($rest)*);
            let res = match gate.2 {
                b'O' => i1 | i2,
                b'A' => i1 & i2,
                _ => i1 ^ i2,
            };
            WIRES[gate.3] = res;
            res
        }
    }};
}

        let mut z = 0;
        for z_idx in 0..46 {
            z |= (calc_unrolled!(15600 + z_idx, a a a a a) as u64) << z_idx;
        }
        z
    }
}

static mut WIRES_INPUT_INTO: [[bool; 3]; 15500] = [[false; 3]; 15500];

#[inline(always)]
unsafe fn outputs_into(output: usize, gate_type: u8) -> bool {
    output < 15500
        && *WIRES_INPUT_INTO
            .get_unchecked(output)
            .get_unchecked(GATE_TYPE_LUT[gate_type as usize])
}

const GATE_TYPE_LUT: [usize; 100] = {
    let mut table = [0; 100];
    table[b'X' as usize] = 2;
    table[b'O' as usize] = 1;
    table[b'A' as usize] = 0;
    table
};

#[inline(always)]
fn sort_groups(slice: &mut [u8]) {
    assert_eq!(slice.len(), 31);
    let mut indices: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    indices.sort_unstable_by(|&a, &b| {
        let group_a = &slice[a * 4..a * 4 + 3];
        let group_b = &slice[b * 4..b * 4 + 3];
        group_a.cmp(group_b)
    });
    (0..=7).for_each(|i| {
        if indices[i] != i {
            for j in 0..3 {
                slice.swap(i * 4 + j, indices[i] * 4 + j);
            }
            let swapped_index = indices[i];
            indices[indices.iter().position(|&x| x == i).unwrap()] = swapped_index;
            indices[i] = i;
        }
    });
}

pub fn part2(input: &str) -> &str {
    unsafe {
        let input = input.as_bytes();
        let mut input = input.as_ptr();

        fn b26(val: [u8; 3]) -> usize {
            let [a, b, c] = [
                (val[0] - b'a') as usize,
                (val[1] - b'a') as usize,
                (val[2] - b'a') as usize,
            ];
            a * 26 * 26 + b * 26 + c
        }

        input = input.add("x00: ".len());
        input = input.add("0\nx00: ".len() * 45);
        input = input.add("0\ny00: ".len() * 45);
        input = input.sub("0\ny00: ".len());
        input = input.add("0\n\n".len());
        // gate: (in1,in2,op,out) use 0 as special op for already calculated

        // we interpret abc values as b26, can go up to 15466
        // so we use values above that for the xZZ, yZZ, zZZ (add 15500 plus another 50 to
        // distinguish)

        let mut first_carry = 0;

        (0..222).for_each(|i| {
            let i1 = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                // is x,y,z so do as described above
                (*input.offset(1) as usize - b'0' as usize) * 10
                    + (*input.offset(2) as usize - b'0' as usize)
                    + 15500
                    + 50 * ((*input.offset(0) - b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            let op = *input.offset(4);
            input = input.add(7);
            if op != b'O' {
                input = input.add(1);
            }
            let i2 = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                (*input.offset(1) as usize - b'0' as usize) * 10
                    + (*input.offset(2) as usize - b'0' as usize)
                    + 15500
                    + 50 * ((*input.offset(0) - b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            input = input.add(7);
            let out = if (*input.offset(0)).wrapping_sub(b'x') < 3 {
                (*input.offset(1) as usize - b'0' as usize) * 10
                    + (*input.offset(2) as usize - b'0' as usize)
                    + 15500
                    + 50 * ((*input.offset(0) - b'x') as usize)
            } else {
                b26([*input.offset(0), *input.offset(1), *input.offset(2)])
            };
            input = input.add(4);
            if i1 == 15500 && i2 == 15550 && op == b'A' {
                first_carry = out;
            }

            if i1 < 15500 {
                WIRES_INPUT_INTO.get_unchecked_mut(i1)[GATE_TYPE_LUT[op as usize]] = true;
            }
            if i2 < 15500 {
                WIRES_INPUT_INTO.get_unchecked_mut(i2)[GATE_TYPE_LUT[op as usize]] = true;
            }

            GATES[i] = (i1, i2, op, out);
        });

        static mut SWAPPED_WIRES: [u8; 31] = [
            0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0,
            b',', 0, 0, 0, b',', 0, 0, 0,
        ];

        let mut swapped_wires_found = 0;

        fn reverse_b26(mut value: usize) -> [u8; 3] {
            if value >= 15500 {
                let norm = value - 15500;
                let idx = norm % 50;
                [
                    (norm / 50) as u8 + b'x',
                    (idx / 10) as u8 + b'0',
                    (idx % 10) as u8 + b'0',
                ]
            } else {
                let c = (value % 26) as u8;
                value /= 26;
                let b = (value % 26) as u8;
                value /= 26;
                let a = value as u8;

                [a + b'a', b + b'a', c + b'a']
            }
        }

        for i in 0..222 {
            let gate = GATES.get_unchecked(i);

            if gate.3 == 15600 || gate.3 == 15645 || gate.3 == first_carry {
                continue;
            }

            if gate.3 >= 15600 && (gate.2 != b'X') {
                let out = reverse_b26(gate.3);
                SWAPPED_WIRES[swapped_wires_found * 4] = out[0];
                SWAPPED_WIRES[swapped_wires_found * 4 + 1] = out[1];
                SWAPPED_WIRES[swapped_wires_found * 4 + 2] = out[2];
                swapped_wires_found += 1;
                if swapped_wires_found >= 8 {
                    break;
                }
                continue;
            }

            if !((gate.2 == b'A'
                && !(gate.0 < 15600 && gate.1 < 15600 && gate.0 >= 15500 && gate.1 >= 15500)
                && outputs_into(gate.3, b'O'))
                || (gate.2 == b'X'
                    && ((outputs_into(gate.3, b'A') && outputs_into(gate.3, b'X'))
                        && (gate.0 < 15600
                            && gate.1 < 15600
                            && gate.0 >= 15500
                            && gate.1 >= 15500))
                    || (gate.3 >= 15600))
                || (gate.2 == b'A'
                    && (gate.0 < 15600 && gate.1 < 15600 && gate.0 >= 15500 && gate.1 >= 15500)
                    && outputs_into(gate.3, b'O'))
                || (gate.2 == b'O'
                    && !(gate.0 < 15600 && gate.1 < 15600 && gate.0 >= 15500 && gate.1 >= 15500)
                    && outputs_into(gate.3, b'A')
                    && outputs_into(gate.3, b'X')))
            {
                let out = reverse_b26(gate.3);
                SWAPPED_WIRES[swapped_wires_found * 4] = out[0];
                SWAPPED_WIRES[swapped_wires_found * 4 + 1] = out[1];
                SWAPPED_WIRES[swapped_wires_found * 4 + 2] = out[2];
                swapped_wires_found += 1;
                if swapped_wires_found >= 8 {
                    break;
                }
                continue;
            }
        }

        sort_groups(&mut SWAPPED_WIRES);
        std::str::from_utf8(&SWAPPED_WIRES).unwrap()
    }
}
