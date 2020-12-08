use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize)
}

impl Instruction {
    fn from_strs(instr: &str, val: &str) -> Self {
        match instr {
            "acc" => Instruction::Acc(val.parse::<i32>().unwrap()),
            "jmp" => Instruction::Jmp(val.parse::<isize>().unwrap()),
            "nop" => Instruction::Nop(val.parse::<isize>().unwrap()),
            _ => panic!(format!("Unknown instruction '{}'", instr)),
        }
    }

    fn act(&self, pc: &mut usize, acc: &mut i32) {
        if let &Instruction::Acc(val) = self {
            *acc += val;
        }
        match self {
            Instruction::Acc(_) | Instruction::Nop(_) => *pc += 1,
            Instruction::Jmp(val) => *pc = (*pc as isize + val) as usize,
        }
    }

    fn is_acc(&self) -> bool {
        match self {
            Instruction::Acc(_) => true,
            _ => false,
        }
    }

    fn switch_jmp_nop(&mut self) {
        match self {
            Instruction::Jmp(val) => *self = Instruction::Nop(*val),
            Instruction::Nop(val) => *self = Instruction::Jmp(*val),
            _ => {}
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(usize, Instruction)> {
    input
        .lines()
        .map(|line| {
            let (instr, rest) = line.split_at(3);
            let (_, amt) = rest.split_at(1);
            (0, Instruction::from_strs(instr, amt))
        })
        .collect::<Vec<_>>()
}

#[aoc(day8, part1)]
pub fn part1(input: &[(usize, Instruction)]) -> i32 {
    let mut prog = input.to_vec();
    let mut acc = 0;
    let mut pc = 0;
    loop {
        let (exec_count, instr) = &mut prog[pc];
        if *exec_count == 1 {
            break;
        } else {
            instr.act(&mut pc, &mut acc);
            *exec_count += 1;
        }
    }
    acc
}

#[aoc(day8, part2)]
pub fn part2(input: &[(usize, Instruction)]) -> i32 {
    let mut prog = input.to_vec();
    for (ind, _) in input.iter().enumerate().filter(|(_, (_, ins))| !ins.is_acc()) {
        prog[ind].1.switch_jmp_nop();
        let mut acc = 0;
        let mut pc = 0;
        loop {
            let (exec_count, instr) = &mut prog[pc];
            if *exec_count == 1 {
                break;
            } else {
                instr.act(&mut pc, &mut acc);
                *exec_count += 1;
                if pc == prog.len() {
                    return acc;
                } else if pc > prog.len() {
                    break;
                }
            }
        }
        prog.iter_mut().for_each(|(count, _)| *count = 0);
        prog[ind].1.switch_jmp_nop();
    }
    panic!("No solution found.");
}
