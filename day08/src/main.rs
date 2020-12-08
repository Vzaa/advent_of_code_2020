use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Cpu {
    pc: i64,
    acc: i64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { pc: 0, acc: 0 }
    }

    fn run_op(&mut self, mem: &[(Op, i64)]) -> bool {
        let op = if let Some(o) = mem.get(self.pc as usize) {
            o
        } else {
            return false;
        };

        match op {
            (Op::Nop, _) => self.pc += 1,
            (Op::Acc, v) => {
                self.acc += v;
                self.pc += 1
            }
            (Op::Jmp, v) => self.pc += v,
        }

        true
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
        if !pc_hist.insert(cpu.pc) {
            println!("Part 1: {}", cpu.acc);
            return;
        }

        if !cpu.run_op(&mem) {
            return;
        }
    }
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mem_org: Vec<_> = instr
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let op: Op = iter.next().unwrap().parse().unwrap();
            let val: i64 = iter.next().unwrap().parse().unwrap();
            (op, val)
        })
        .collect();

    let mem_iter = mem_org
        .iter()
        .enumerate()
        .filter_map(|(idx, op)| match op.0 {
            Op::Nop => {
                let mut tmp = mem_org.clone();
                tmp[idx].0 = Op::Jmp;
                Some(tmp)
            }
            Op::Jmp => {
                let mut tmp = mem_org.clone();
                tmp[idx].0 = Op::Nop;
                Some(tmp)
            }
            _ => None,
        });

    for mem in mem_iter {
        let mut cpu = Cpu::new();
        let mut pc_hist = HashSet::new();

        loop {
            if !pc_hist.insert(cpu.pc) {
                break;
            }

            if !cpu.run_op(&mem) {
                println!("Part 2: {}", cpu.acc);
                return;
            }
        }
    }
}

fn main() {
    p1();
    p2();
}
