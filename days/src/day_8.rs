use aoc_runner_derive::aoc;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize)
}

impl Instruction {
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

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let mut prog = input
        .lines()
        .map(|line| {
            let (instr, rest) = line.split_at(3);
            let (_, amt) = rest.split_at(1);
            match instr {
                "acc" => (0, Instruction::Acc(amt.parse::<i32>().unwrap())),
                "jmp" => (0, Instruction::Jmp(amt.parse::<isize>().unwrap())),
                "nop" => (0, Instruction::Nop(amt.parse::<isize>().unwrap())),
                _ => panic!(format!("Unknown instruction '{}'", instr)),
            }
        })
        .collect::<Vec<_>>();
    let mut acc = 0;
    let mut pc = 0;
    loop {
        let (exec_count, instr) = &mut prog[pc];
        if *exec_count == 1 {
            break;
        } else {
            match *instr {
                Instruction::Acc(amt) => {
                    acc += amt;
                    pc += 1;
                },
                Instruction::Jmp(amt) => pc = (pc as isize + amt) as usize,
                Instruction::Nop(_) => pc += 1,
            }
            *exec_count += 1;
        }
    }
    acc
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let mut prog = input
        .lines()
        .map(|line| {
            let (instr, rest) = line.split_at(3);
            let (_, amt) = rest.split_at(1);
            match instr {
                "acc" => (0, Instruction::Acc(amt.parse::<i32>().unwrap())),
                "jmp" => (0, Instruction::Jmp(amt.parse::<isize>().unwrap())),
                "nop" => (0, Instruction::Nop(amt.parse::<isize>().unwrap())),
                _ => panic!(format!("Unknown instruction '{}'", instr)),
            }
        })
        .collect::<Vec<_>>();
    // Do jmp => nop pass
    for (ind, _) in prog.iter().enumerate().filter(|(_, (_, ins))| !ins.is_acc()) {
        let mut prog_clone = prog.clone();
        prog_clone[ind].1.switch_jmp_nop();
        let mut acc = 0;
        let mut pc = 0;
        loop {
            let (exec_count, instr) = &mut prog_clone[pc];
            if *exec_count == 1 {
                break;
            } else {
                match *instr {
                    Instruction::Acc(amt) => {
                        acc += amt;
                        pc += 1;
                    },
                    Instruction::Jmp(amt) => pc = (pc as isize + amt) as usize,
                    Instruction::Nop(_) => pc += 1,
                }
                *exec_count += 1;
                if pc == prog_clone.len() {
                    return acc;
                } else if pc > prog_clone.len() {
                    break;
                }
            }
        }
    }
    panic!("No solution found.");
}
