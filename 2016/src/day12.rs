type Code = Vec<Instruction>;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum RegOrValue {
    Register(Register),
    Value(i32),
}

#[derive(Debug)]
pub enum Instruction {
    Copy(RegOrValue, Register),
    Inc(Register),
    Dec(Register),
    Jnz(RegOrValue, i32),
}

fn compute(mut regs: [i32; 4], code: &Code) -> [i32; 4] {
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
                if cond != 0 {
                    pc += *offset as usize;
                    continue;
                }
            }
        }
        pc += 1;
    }
    regs
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Code {
    input
        .lines()
        .map(|line| {
            if line.starts_with("cpy") {
                let (a, b) = line.split_once(" ").unwrap().1.split_once(" ").unwrap();
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                let b = Register::parse(b).unwrap();
                Instruction::Copy(a, b)
            } else if line.starts_with("inc") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a).unwrap();
                Instruction::Inc(a)
            } else if line.starts_with("dec") {
                let a = line.split_once(" ").unwrap().1;
                let a = Register::parse(a).unwrap();
                Instruction::Dec(a)
            } else if line.starts_with("jnz") {
                let (a, b) = line.split_once(" ").unwrap().1.split_once(" ").unwrap();
                let a = Register::parse(a)
                    .map(RegOrValue::Register)
                    .unwrap_or_else(|| RegOrValue::Value(a.parse().unwrap()));
                let b = b.parse().unwrap();
                Instruction::Jnz(a, b)
            } else {
                unreachable!()
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(code: &Code) -> usize {
    let regs = compute([0_i32; 4], code);
    regs[0] as _
}

#[aoc(day12, part2)]
pub fn part2(code: &Code) -> usize {
    let regs = compute([0, 0, 1, 0], code);
    regs[0] as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            super::part1(
                &generator("cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a")
            ),
            42
        )
    }

    #[test]
    fn part2() {}
}
