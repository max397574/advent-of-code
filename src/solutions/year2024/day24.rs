use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum GateType {
    And,
    Xor,
    Or,
}

impl GateType {
    fn from_str(val: &str) -> Self {
        match val {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Gate {
    gate_type: GateType,
    inputs: (String, String),
    output: String,
}

// utilities for p2
impl Gate {
    fn has_direct_input(&self) -> bool {
        (self.inputs.0.starts_with('x') && self.inputs.1.starts_with('y'))
            || (self.inputs.1.starts_with('x') && self.inputs.0.starts_with('y'))
    }

    fn has_direct_output(&self) -> bool {
        self.output.starts_with('z')
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

    fn has_input(&self, input: &str) -> bool {
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
    let mut wires = HashMap::new();
    raw_wires.lines().for_each(|line| {
        wires.insert(line[0..3].to_string(), line.chars().nth(5).unwrap() == '1');
    });

    let mut gates = Vec::new();

    raw_gates.lines().for_each(|line| {
        let (gate, output) = line.split_once(" -> ").unwrap();
        let (input1, rest) = gate.split_once(" ").unwrap();
        let (gate_type, input2) = rest.split_once(" ").unwrap();
        gates.push(Gate {
            gate_type: GateType::from_str(gate_type),
            inputs: (input1.to_owned(), input2.to_owned()),
            output: output.to_owned(),
        })
    });

    loop {
        let mut all_set = true;
        for gate in gates.clone() {
            if let Some(&input1) = wires.get(&gate.inputs.0)
                && let Some(&input2) = wires.get(&gate.inputs.1)
            {
                *wires.entry(gate.output).or_insert(false) = {
                    match gate.gate_type {
                        GateType::Or => input1 || input2,
                        GateType::And => input1 && input2,
                        GateType::Xor => input1 ^ input2,
                    }
                };
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
        if let Some(rest) = name.strip_prefix('z') {
            let idx = rest.parse::<u64>().unwrap();
            if val {
                outval |= 1 << idx;
            }
        }
    }
    outval
}

pub fn part2(input: &str) -> String {
    let (_, raw_gates) = input.split_once("\n\n").unwrap();
    let mut gates = Vec::new();

    let mut first_carry = "";

    raw_gates.lines().for_each(|line| {
        let (gate, output) = line.split_once(" -> ").unwrap();
        let (input1, rest) = gate.split_once(" ").unwrap();
        let (gate_type, input2) = rest.split_once(" ").unwrap();
        if input1 == "x00" && input2 == "y00" && gate_type == "AND" {
            first_carry = output;
        }
        gates.push(Gate {
            gate_type: GateType::from_str(gate_type),
            inputs: (input1.to_owned(), input2.to_owned()),
            output: output.to_owned(),
        })
    });

    let mut swapped_wires = Vec::new();

    for gate in &gates {
        // there are five different gates in an adder
        // - XOR with direct input, which outputs to other XOR and AND
        // - XOR with direct output with input from carry and from other XOR
        // - AND with direct inputs which outputs to OR
        // - AND with inputs from carry and XOR which outputs to OR
        // - OR with carry output with inputs from ANDs
        if gate.output == "z00" || gate.output == "z45" || gate.output == first_carry {
            continue;
        }

        if gate.has_direct_output() && !gate.is_xor() {
            swapped_wires.push(gate.output.clone());
            continue;
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
            swapped_wires.push(gate.output.clone());
            continue;
        }
    }

    swapped_wires.sort();
    swapped_wires.join(",")
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
        assert_eq!(part2(INPUT2).to_string(), String::from("z00,z01,z02,z05"))
    }
}
