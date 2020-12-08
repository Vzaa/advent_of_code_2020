use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
struct Cpu {
    pc: i64,
    acc: i64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { pc: 0, acc: 0 }
    }

    fn run_op(&mut self, op: &(Op, i64)) {
        match op {
            (Op::Nop, _) => self.pc += 1,
            (Op::Acc, v) => {
                self.acc += v;
                self.pc += 1
            }
            (Op::Jmp, v) => self.pc += v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => return Err(()),
        };
        Ok(op)
    }
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mem: Vec<_> = instr
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let op: Op = iter.next().unwrap().parse().unwrap();
            let val: i64 = iter.next().unwrap().parse().unwrap();
            (op, val)
        })
        .collect();

    let mut cpu = Cpu::new();
    let mut pc_hist = HashSet::new();

    loop {
        let op = &mem[cpu.pc as usize];

        if !pc_hist.insert(cpu.pc) {
            println!("Part 1: {}", cpu.acc);
            return;
        }

        cpu.run_op(op);
    }
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mem: Vec<_> = instr
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let op: Op = iter.next().unwrap().parse().unwrap();
            let val: i64 = iter.next().unwrap().parse().unwrap();
            (op, val)
        })
        .collect();

    for (idx, op) in mem.iter().enumerate() {
        let new_mem = if op.0 == Op::Nop {
            let mut tmp = mem.clone();
            tmp[idx].0 = Op::Jmp;
            tmp
        } else if op.0 == Op::Jmp {
            let mut tmp = mem.clone();
            tmp[idx].0 = Op::Nop;
            tmp
        } else {
            continue;
        };

        let mut cpu = Cpu::new();
        let mut pc_hist = HashSet::new();

        loop {
            let op = if let Some(o) = new_mem.get(cpu.pc as usize) {
                o
            } else {
                println!("Part 2: {}", cpu.acc);
                return;
            };

            if !pc_hist.insert(cpu.pc) {
                break;
            }

            cpu.run_op(op);
        }
    }
}

fn main() {
    p1();
    p2();
}
