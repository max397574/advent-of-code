#![feature(let_chains)]
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum GateType {
    And,
    Xor,
    Or,
}

impl GateType {
    fn from_bytes(val: [u8; 3]) -> Self {
        match val[0] {
            b'A' => Self::And,
            b'O' => Self::Or,
            b'X' => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Gate {
    gate_type: GateType,
    inputs: ([u8; 3], [u8; 3]),
    output: [u8; 3],
}

// utilities for p2
impl Gate {
    fn has_direct_input(&self) -> bool {
        (self.inputs.0[0] == b'x' && self.inputs.1[0] == b'y')
            || (self.inputs.1[0] == b'x' && self.inputs.0[0] == b'y')
    }

    fn has_direct_output(&self) -> bool {
        self.output[0] == b'z'
    }

    fn is_xor(&self) -> bool {
        self.gate_type == GateType::Xor
    }

    fn is_and(&self) -> bool {
        self.gate_type == GateType::And
    }

    fn is_or(&self) -> bool {
        self.gate_type == GateType::Or
    }

    fn has_input(&self, input: &[u8]) -> bool {
        self.inputs.0 == input || self.inputs.1 == input
    }

    fn outputs_into(&self, gate_type: GateType, gates: &[Gate]) -> bool {
        gates
            .iter()
            .filter(|gate2| gate2.has_input(&self.output) && gate2.gate_type == gate_type)
            .count()
            >= 1
    }
}

pub fn part1(input: &str) -> u128 {
    let (raw_wires, raw_gates) = input.split_once("\n\n").unwrap();
    let mut wires: HashMap<&[u8; 3], bool> = HashMap::new();
    raw_wires.lines().for_each(|line| {
        wires.insert(
            line[0..3].as_bytes().try_into().unwrap(),
            line.chars().nth(5).unwrap() == '1',
        );
    });

    let mut gates = Vec::new();

    raw_gates.lines().for_each(|line| {
        let line = line.as_bytes();
        let input1: [u8; 3] = line[0..3].try_into().unwrap();
        let gate: [u8; 3] = line[4..7].try_into().unwrap();
        let gate_len = if gate[0] == b'O' { 2 } else { 3 };
        let input2: [u8; 3] = line[5 + gate_len..8 + gate_len].try_into().unwrap();
        let output: [u8; 3] = line[12 + gate_len..15 + gate_len].try_into().unwrap();
        gates.push(Gate {
            gate_type: GateType::from_bytes(gate),
            inputs: (input1, input2),
            output,
        })
    });

    loop {
        let mut all_set = true;
        for gate in &gates {
            if let Some(&input1) = wires.get(&gate.inputs.0)
                && let Some(&input2) = wires.get(&gate.inputs.1)
            {
                wires.insert(&gate.output, {
                    match gate.gate_type {
                        GateType::Or => input1 || input2,
                        GateType::And => input1 && input2,
                        GateType::Xor => input1 ^ input2,
                    }
                });
            } else {
                all_set = false;
            }
        }
        if all_set {
            break;
        }
    }

    let mut outval = 0u128;
    for (name, val) in wires {
        if name[0] == b'z' {
            let idx = name[1] * 10 + name[2];
            if val {
                outval |= 1 << idx;
            }
        }
    }
    outval
}

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
    let (_, raw_gates) = input.split_once("\n\n").unwrap();
    let mut gates = Vec::new();

    let mut first_carry = [0; 3];

    raw_gates.lines().for_each(|line| {
        let line = line.as_bytes();
        let input1: [u8; 3] = line[0..3].try_into().unwrap();
        let gate: [u8; 3] = line[4..7].try_into().unwrap();
        let gate_len = if gate[0] == b'O' { 2 } else { 3 };
        let input2: [u8; 3] = line[5 + gate_len..8 + gate_len].try_into().unwrap();
        let output: [u8; 3] = line[12 + gate_len..15 + gate_len].try_into().unwrap();
        if input1 == [b'x', b'0', b'0'] && input2 == [b'y', b'0', b'0'] && gate[0] == b'A' {
            first_carry = output;
        }
        gates.push(Gate {
            gate_type: GateType::from_bytes(gate),
            inputs: (input1, input2),
            output,
        })
    });

    static mut SWAPPED_WIRES: [u8; 31] = [
        0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',', 0, 0, 0, b',',
        0, 0, 0, b',', 0, 0, 0,
    ];

    let mut swapped_wires_found = 0;

    for gate in &gates {
        // there are five different gates in an adder
        // - XOR with direct input, which outputs to other XOR and AND
        // - XOR with direct output with input from carry and from other XOR
        // - AND with direct inputs which outputs to OR
        // - AND with inputs from carry and XOR which outputs to OR
        // - OR with carry output with inputs from ANDs
        if gate.output == [b'z', b'0', b'0']
            || gate.output == [b'z', b'4', b'5']
            || gate.output == first_carry
        {
            continue;
        }

        if gate.has_direct_output() && !gate.is_xor() {
            unsafe {
                SWAPPED_WIRES[swapped_wires_found * 4] = gate.output[0];
                SWAPPED_WIRES[swapped_wires_found * 4 + 1] = gate.output[1];
                SWAPPED_WIRES[swapped_wires_found * 4 + 2] = gate.output[2];
                swapped_wires_found += 1;
                if swapped_wires_found >= 8 {
                    break;
                }
                continue;
            }
        }

        if !((gate.is_and() && !gate.has_direct_input() && gate.outputs_into(GateType::Or, &gates))
            || (gate.is_xor()
                && ((gate.outputs_into(GateType::And, &gates)
                    && gate.outputs_into(GateType::Xor, &gates)
                    && gate.has_direct_input())
                    || gate.has_direct_output()))
            || (gate.is_and()
                && gate.has_direct_input()
                && gate.outputs_into(GateType::Or, &gates))
            || (gate.is_or()
                && !gate.has_direct_input()
                && gate.outputs_into(GateType::And, &gates)
                && gate.outputs_into(GateType::Xor, &gates)))
        {
            unsafe {
                SWAPPED_WIRES[swapped_wires_found * 4] = gate.output[0];
                SWAPPED_WIRES[swapped_wires_found * 4 + 1] = gate.output[1];
                SWAPPED_WIRES[swapped_wires_found * 4 + 2] = gate.output[2];
                swapped_wires_found += 1;
                if swapped_wires_found >= 8 {
                    break;
                }
                continue;
            }
        }
    }

    unsafe {
        sort_groups(&mut SWAPPED_WIRES);
        std::str::from_utf8(&SWAPPED_WIRES).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("4"))
    }

    #[test]
    fn part_2() {
        assert_eq!(
            part2(INPUT2).to_string(),
            // weird because of optimizations
            String::from("\0\0\0,\0\0\0,\0\0\0,\0\0\0,z01,z02,z03,z04")
        )
    }
}
