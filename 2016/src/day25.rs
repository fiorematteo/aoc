#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn parse(x: &str) -> Option<Self> {
        match x {
            "a" => Some(Register::A),
            "b" => Some(Register::B),
            "c" => Some(Register::C),
            "d" => Some(Register::D),
            _ => None,
        }
    }

    fn offset(&self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RegOrValue {
    Register(Register),
    Value(i32),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Instruction {
    Copy(RegOrValue, Register),
    Inc(Register),
    Dec(Register),
    Jnz(RegOrValue, RegOrValue),
    Tgl(RegOrValue),
    Nop(RegOrValue, Option<RegOrValue>, String),
    Out(RegOrValue),
}

fn compute(mut regs: [i32; 4], code: &[Instruction]) -> ([i32; 4], Vec<i32>) {
    let mut signals = Vec::new();

    let mut code = code.to_vec();
    let mut pc = 0;
    while pc < code.len() {
        match &code[pc] {
            Instruction::Copy(reg_or_value, register) => {
                let from = match reg_or_value {
                    RegOrValue::Register(register) => regs[register.offset()],
                    RegOrValue::Value(v) => *v,
                };
                regs[register.offset()] = from;
            }
            Instruction::Inc(register) => regs[register.offset()] += 1,
            Instruction::Dec(register) => regs[register.offset()] -= 1,
            Instruction::Jnz(reg_or_value, offset) => {
                let cond = match reg_or_value {
                    RegOrValue::Register(register) => regs[register.offset()],
                    RegOrValue::Value(v) => *v,
                };
                let offset = match offset {
                    RegOrValue::Register(register) => regs[register.offset()],
                    RegOrValue::Value(v) => *v,
                };
                if cond != 0 {
                    pc = (pc as i32 + offset) as usize;
                    continue;
                }
            }
            Instruction::Tgl(reg_or_value) => {
                let offset = match reg_or_value {
                    RegOrValue::Register(register) => regs[register.offset()],
                    RegOrValue::Value(v) => *v,
                };
                let pointer = pc as i32 + offset;
                if pointer < 0 || pointer as usize >= code.len() {
                    pc += 1;
                    continue;
                }
                let pointer = pointer as usize;
                let new_instr = match code[pointer].clone() {
                    Instruction::Inc(register) => Instruction::Dec(register),
                    Instruction::Dec(register) => Instruction::Inc(register),
                    Instruction::Tgl(reg_or_value) => match reg_or_value {
                        RegOrValue::Register(register) => Instruction::Inc(register),
                        RegOrValue::Value(value) => {
                            Instruction::Nop(RegOrValue::Value(value), None, "inc".to_string())
                        }
                    },
                    Instruction::Copy(reg_or_value, register) => {
                        Instruction::Jnz(reg_or_value, RegOrValue::Register(register))
                    }
                    Instruction::Jnz(reg_or_value, b) => match b {
                        RegOrValue::Register(register) => Instruction::Copy(reg_or_value, register),
                        RegOrValue::Value(b) => Instruction::Nop(
                            reg_or_value,
                            Some(RegOrValue::Value(b)),
                            "cpy".to_string(),
                        ),
                    },
                    Instruction::Nop(a, b, name) => match name.as_str() {
                        "inc" => Instruction::Nop(a, b, "dec".to_string()),
                        "dec" => Instruction::Nop(a, b, "inc".to_string()),
                        "cpy" => Instruction::Jnz(a, b.unwrap()),
                        _ => unreachable!(),
                    },
                    Instruction::Out(..) => unimplemented!(),
                };
                code[pointer] = new_instr;
            }
            Instruction::Nop(..) => (),
            Instruction::Out(reg_or_value) => {
                let signal = match reg_or_value {
                    RegOrValue::Register(register) => regs[register.offset()],
                    RegOrValue::Value(v) => *v,
                };
                signals.push(signal);
                if signals.len() > 1000 {
                    break;
                }
            }
        }
        pc += 1;
    }
    (regs, signals)
}

#[aoc_generator(day25)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("cpy") {
                let (a, b) = line.split_once(" ").unwrap().1.split_once(" ").unwrap();
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                let b = Register::parse(b)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(b.parse().unwrap()));
                match (a, b) {
                    (a, RegOrValue::Register(b)) => Instruction::Copy(a, b),
                    (a, b) => Instruction::Nop(a, Some(b), "cpy".to_string()),
                }
            } else if line.starts_with("inc") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                match a {
                    RegOrValue::Register(a) => Instruction::Inc(a),
                    a => Instruction::Nop(a, None, "inc".to_string()),
                }
            } else if line.starts_with("dec") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                match a {
                    RegOrValue::Register(a) => Instruction::Dec(a),
                    a => Instruction::Nop(a, None, "dec".to_string()),
                }
            } else if line.starts_with("jnz") {
                let (a, b) = line.split_once(" ").unwrap().1.split_once(" ").unwrap();
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                let b = Register::parse(b)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(b.parse().unwrap()));
                Instruction::Jnz(a, b)
            } else if line.starts_with("tgl") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                Instruction::Tgl(a)
            } else if line.starts_with("out") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                Instruction::Out(a)
            } else {
                unreachable!()
            }
        })
        .collect()
}

#[aoc(day25, part1)]
pub fn part1(code: &[Instruction]) -> i32 {
    for i in 0.. {
        let (_, signal) = compute([i, 0, 0, 0], code);
        let good_signal = signal.iter().enumerate().all(|(i, x)| x % 2 == i as i32 % 2 )
            || signal.iter().enumerate().all(|(i, x)| x % 2 != i as i32 % 2);
        if good_signal {
            return i;
        }
    }
    unreachable!()
}
