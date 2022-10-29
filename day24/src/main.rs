use crate::instruction::Program;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> u64 {
    let program = Program::from(input);
    program.exec()
}

mod instruction {
    use std::borrow::{Borrow, BorrowMut};
    use std::cmp::Ordering;

    use itertools::Itertools;

    use crate::register::{Registers, Var};

    #[derive(Copy, Clone)]
    pub enum Instruction {
        Inp(Var, u8),
        Add(Var, Var),
        Mul(Var, Var),
        Div(Var, Var),
        Mod(Var, Var),
        Eql(Var, Var),
    }

    pub struct Program(Vec<Instruction>);

    impl From<&str> for Program {
        fn from(instructions: &str) -> Self {
            let program = instructions
                .lines()
                .map(|l| {
                    let parts = l.split_whitespace().collect::<Vec<_>>();
                    match parts[0] {
                        "inp" => Instruction::Inp(Var::from(parts[1]), 0),
                        "add" => Instruction::Add(Var::from(parts[1]), Var::from(parts[2])),
                        "mul" => Instruction::Mul(Var::from(parts[1]), Var::from(parts[2])),
                        "div" => Instruction::Div(Var::from(parts[1]), Var::from(parts[2])),
                        "mod" => Instruction::Mod(Var::from(parts[1]), Var::from(parts[2])),
                        "eql" => Instruction::Eql(Var::from(parts[1]), Var::from(parts[2])),
                        _ => panic!("could not parse input src"),
                    }
                })
                .collect::<Vec<_>>();

            if program.len() != 252 {
                panic!(
                    "malformed ALU program (expected 252 instructions, found {}",
                    program.len()
                );
            }

            Self(program)
        }
    }

    impl Program {
        pub fn exec(&self) -> u64 {
            let mut states = vec![(Registers::new(), 0u64)];

            for i in self.0.iter() {
                if let Instruction::Inp(reg, _) = i {
                    println!("Processing {} states...", states.len());

                    states = states
                        .iter_mut()
                        .map(|s| {
                            s.0.load(&[0, 0, 0, s.0.get(&Var::from('z'))]);
                            *s
                        })
                        .sorted_unstable_by(|(ar, an), (br, bn)| match ar.cmp(br) {
                            Ordering::Equal => bn.cmp(an),
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                        })
                        .dedup_by(|(ar, _), (br, _)| ar == br)
                        .collect_vec();

                    let mut next_states = Vec::with_capacity(states.len() * 9);

                    for (mut r, n) in states.iter() {
                        for d in 1..=9u8 {
                            r.apply(&Instruction::Inp(*reg, d));
                            next_states.push((r, n * 10 + d as u64));
                        }
                    }

                    states = next_states;
                } else {
                    for (r, _) in states.iter_mut() {
                        r.apply(&i);
                    }
                }
            }

            println!("Finished with {} states...", states.len());

            *states
                .iter()
                .filter_map(|(r, n)| (r.get(&Var::from('z')) == 0).then_some(n).or(None))
                .max()
                .unwrap()
        }
    }
}

pub mod register {
    use std::str::FromStr;

    use crate::instruction::Instruction;

    #[derive(Copy, Clone)]
    pub enum Var {
        Reg(usize),
        Num(isize),
    }

    impl From<char> for Var {
        fn from(value: char) -> Self {
            match value {
                'w' => Self::Reg(Registers::W),
                'x' => Self::Reg(Registers::X),
                'y' => Self::Reg(Registers::Y),
                'z' => Self::Reg(Registers::Z),
                _ => Self::Num(isize::try_from(value.to_digit(10).unwrap()).unwrap()),
            }
        }
    }

    impl From<&str> for Var {
        fn from(value: &str) -> Self {
            if value.len() == 1 {
                Self::from(char::from_str(value).unwrap())
            } else {
                Self::Num(value.parse().unwrap())
            }
        }
    }

    type Mem = [isize; 4];

    #[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
    pub struct Registers(Mem);

    impl Registers {
        pub const W: usize = 0;
        pub const X: usize = 1;
        pub const Y: usize = 2;
        pub const Z: usize = 3;

        pub fn new() -> Self {
            Self([0; 4])
        }

        pub fn get(&self, reg: &Var) -> isize {
            match reg {
                Var::Reg(reg) => self.0[*reg],
                Var::Num(num) => *num,
            }
        }

        pub fn set(&mut self, reg: &Var, num: isize) {
            if let Var::Reg(reg) = reg {
                self.0[*reg] = num;
                return;
            }

            panic!("invalid register access");
        }

        pub fn dump(&self) -> Mem {
            self.0
        }

        pub fn load(&mut self, mem: &Mem) {
            self.0 = *mem;
        }

        pub fn apply(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::Inp(a, digit) => self.set(a, *digit as isize),
                Instruction::Add(a, b) => self.set(a, self.get(a) + self.get(b)),
                Instruction::Mul(a, b) => self.set(a, self.get(a) * self.get(b)),
                Instruction::Div(a, b) => self.set(a, self.get(a) / self.get(b)),
                Instruction::Mod(a, b) => self.set(a, self.get(a) % self.get(b)),
                Instruction::Eql(a, b) => self.set(a, (self.get(a) == self.get(b)) as isize),
            }
        }
    }
}
