use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
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

#[derive(Clone, Debug)]
pub struct Program {
    instructions: Vec<(bool, Instruction)>,
    acc: i32,
    pc: usize,
}

impl Program {
    fn new(instructions: Vec<(bool, Instruction)>) -> Self {
        Program {
            instructions,
            acc: 0,
            pc: 0,
        }
    }

    fn execute_until<F: Fn(&Program) -> bool, T, R: FnOnce(&Program) -> T>(
        &mut self,
        until: F,
        ret: R,
    ) -> T {
        while !until(&self) {
            let pc = self.pc;
            self.instructions[pc].1.act(&mut self.pc, &mut self.acc);
            self.instructions[pc].0 = true;
        }
        ret(&self)
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
        self.instructions
            .iter_mut()
            .for_each(|(executed, _)| *executed = false);
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Program {
    let instructions = input
        .lines()
        .map(|line| {
            let (instr, rest) = line.split_at(3);
            let (_, amt) = rest.split_at(1);
            (false, Instruction::from_strs(instr, amt))
        })
        .collect::<Vec<_>>();
    Program::new(instructions)
}

#[aoc(day8, part1)]
pub fn part1(input: &Program) -> i32 {
    input
        .clone()
        .execute_until(|prog| prog.instructions[prog.pc].0, |prog| prog.acc)
}

#[aoc(day8, part2)]
pub fn part2(input: &Program) -> i32 {
    let mut program = input.clone();
    input
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, (_, ins))| !ins.is_acc())
        .find_map(|(i, _)| {
            program.instructions[i].1.switch_jmp_nop();
            let (end_acc, end_pc) = program.execute_until(
                |prog| prog.pc >= input.instructions.len() || prog.instructions[prog.pc].0,
                |prog| (prog.acc, prog.pc),
            );
            if end_pc == input.instructions.len() {
                Some(end_acc)
            } else {
                program.reset();
                program.instructions[i].1.switch_jmp_nop();
                None
            }
        })
        .unwrap()
}
