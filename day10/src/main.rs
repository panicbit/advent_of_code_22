use aoc::aoc;

#[aoc(2022, 10, 1)]
fn main(input: &str) -> usize {
    let code = input.lines().map(Instr::parse).collect::<Vec<_>>();
    let mut cpu = Cpu::new(code);
    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    let mut total_signal = 0;

    for _ in 0..220 {
        if interesting_cycles.contains(&cpu.cycle) {
            let signal = cpu.cycle * cpu.x as usize;

            total_signal += signal;
        }
        
        cpu.cycle();
    }

    total_signal
}

struct Cpu {
    ip: usize,
    cycle: usize,
    code: Vec<Instr>,
    executing: Option<ExecutingInstr>,
    x: i32,
}

impl Cpu {
    fn new(code: Vec<Instr>) -> Self {
        Self {
            ip: 0,
            cycle: 1,
            code,
            executing: None,
            x: 1,
        }
    }

    fn cycle(&mut self) {
        self.schedule_next_instruction();
        self.execute_current_instruction();
        self.cycle += 1;
    }

    fn schedule_next_instruction(&mut self) {
        if self.executing.is_some() {
            return;
        }

        let Some(instr) = self.code.get(self.ip).copied() else {
            return;
        };

        let cycles_left = match instr {
            Instr::AddX(_) => 2,
            Instr::NoOp => 1,
        };

        self.executing = Some(ExecutingInstr { cycles_left, instr });

        self.ip += 1;
    }

    fn execute_current_instruction(&mut self) {
        let Some(executing) = &mut self.executing else {
            return;
        };

        executing.cycles_left -= 1;

        if executing.cycles_left > 0 {
            return;
        }

        match executing.instr {
            Instr::AddX(value) => self.x += value,
            Instr::NoOp => {}
        };

        self.executing = None;
    }
}

struct ExecutingInstr {
    cycles_left: usize,
    instr: Instr,
}

#[derive(Copy, Clone)]
enum Instr {
    AddX(i32),
    NoOp,
}

impl Instr {
    fn parse(s: &str) -> Self {
        if s == "noop" {
            return Self::NoOp;
        }

        let (op, value) = s.split_once(' ').unwrap();

        assert_eq!(op, "addx");

        let value = value.parse::<i32>().unwrap();

        Self::AddX(value)
    }
}
