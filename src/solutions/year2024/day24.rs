use std::intrinsics::unchecked_sub;

static mut WIRES: [u8; 15700] = [3; 15700];
static mut GATES: [(usize, usize, u8, usize); 222] = [(0, 0, 0, 0); 222];

#[inline(always)]
unsafe fn execute(op: u8, i1: usize, i2: usize, out: usize) {
    let i1 = WIRES.get_unchecked(i1);
    let i2 = WIRES.get_unchecked(i2);
    let res = {
        if op == b'O' {
            i1 | i2
        } else if op == b'A' {
            i1 & i2
        } else {
            i1 ^ i2
        }
    };
    *WIRES.get_unchecked_mut(out) = res;
}

pub fn part1(input: &str) -> u64 {
    unsafe {
        let input = input.as_bytes();
        let mut input = input.as_ptr();

        WIRES.fill(u8::MAX);

        unsafe fn b26(val: [u8; 3]) -> usize {
            let [a, b, c] = [
                (unchecked_sub(*val.get_unchecked(0), b'a')) as usize,
                (unchecked_sub(*val.get_unchecked(1), b'a')) as usize,
                (unchecked_sub(*val.get_unchecked(2), b'a')) as usize,
            ];
            a * 26 * 26 + b * 26 + c
        }

        input = input.add("x00: ".len());

        for i in 0..45 {
            *WIRES.get_unchecked_mut(15500 + i) = unchecked_sub(*input.offset(0), b'0');
            input = input.add("0\nx00: ".len());
        }

        for i in 0..45 {
            *WIRES.get_unchecked_mut(15550 + i) = unchecked_sub(*input.offset(0), b'0');
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
            *GATES.get_unchecked_mut(i) = (i1, i2, op, out);
        });

        let mut all_calculated = false;

        while !all_calculated {
            all_calculated = true;
            for i in 0..222 {
                let gate = GATES.get_unchecked(i);
                if gate.2 == 0 {
                    continue;
                }
                if *WIRES.get_unchecked(gate.0) < 3 && *WIRES.get_unchecked(gate.1) < 3 {
                    execute(gate.2, gate.0, gate.1, gate.3);
                    GATES.get_unchecked_mut(i).2 = 0;
                } else {
                    all_calculated = false;
                }
            }
        }
        let mut z = 0;
        for i in 0..46 {
            z |= (*WIRES.get_unchecked(15600 + i) as u64) << i;
        }
        z
    }
}

static mut WIRES_INPUT_INTO: [[bool; 3]; 15500] = [[false; 3]; 15500];

#[inline(always)]
unsafe fn outputs_into(output: usize, gate_type: u8) -> bool {
    output < 15500 && WIRES_INPUT_INTO[output][GATE_TYPE_LUT[gate_type as usize]]
}

const GATE_TYPE_LUT: [usize; 100] = {
    let mut table = [0; 100];
    table[b'X' as usize] = 2;
    table[b'O' as usize] = 1;
    table[b'A' as usize] = 0;
    table
};

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
            //if input1 == [b'x', b'0', b'0'] && input2 == [b'y', b'0', b'0'] && gate[0] == b'A' {

            if i1 == 15500 && i2 == 15550 && op == b'A' {
                first_carry = out;
            }

            if i1 < 15500 {
                WIRES_INPUT_INTO[i1][GATE_TYPE_LUT[op as usize]] = true;
            }
            if i2 < 15500 {
                WIRES_INPUT_INTO[i2][GATE_TYPE_LUT[op as usize]] = true;
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
            // there are five different gates in an adder
            // - XOR with direct input, which outputs to other XOR and AND
            // - XOR with direct output with input from carry and from other XOR
            // - AND with direct inputs which outputs to OR
            // - AND with inputs from carry and XOR which outputs to OR
            // - OR with carry output with inputs from ANDs

            // gate: (in1,in2,op,out) use 0 as special op for already calculated

            let gate = GATES[i];

            if gate.3 == 15600 || gate.3 == 15645 || gate.3 == first_carry {
                continue;
            }

            // has_direct_output: >=15600
            // has_direct_input: both >= 15500 and < 15600
            // outputs into: keep track of into which gate types wires go

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

            //if !((gate.is_and()
            //    && !gate.has_direct_input()
            //    && gate.outputs_into(GateType::Or, &gates))
            //    || (gate.is_xor()
            //        && ((gate.outputs_into(GateType::And, &gates)
            //            && gate.outputs_into(GateType::Xor, &gates)
            //            && gate.has_direct_input())
            //            || gate.has_direct_output()))
            //    || (gate.is_and()
            //        && gate.has_direct_input()
            //        && gate.outputs_into(GateType::Or, &gates))
            //    || (gate.is_or()
            //        && !gate.has_direct_input()
            //        && gate.outputs_into(GateType::And, &gates)
            //        && gate.outputs_into(GateType::Xor, &gates)))

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
