use std::io::{self, BufRead};

#[derive(Clone)]
struct Interpreter {
    program: Vec<Instruction>,
    pc: i32,
    accum: i32
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: i32,

    /// Whether or not this instruction has been executed.
    executed: bool
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Opcode {
    Nop,
    Acc,
    Jmp
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        let split = &mut input.split_whitespace();
        let opcode = split.next().and_then(Opcode::parse)?;
        let operand = split.next().and_then(|operand| operand.parse::<i32>().ok())?;

        Some(Instruction { opcode, operand, executed: false })
    }
}

impl Opcode {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "nop" => Some(Opcode::Nop),
            "acc" => Some(Opcode::Acc),
            "jmp" => Some(Opcode::Jmp),
                _ => None
        }
    }
}

impl Interpreter {
    /// Parses the current line into an opcode. Returns the bad instruction on failure.
    fn parse(input: impl Iterator<Item=String>) -> Result<Self, String> {
        let program = input.map(|line| Instruction::parse(&line).ok_or(line))
            .collect::<Result<Vec<Instruction>,String>>()?;
        Ok(Interpreter { program, pc: 0, accum: 0 })
    }

    /// Returns Ok on successful termination, or Err on an infinite loop.
    fn run(&mut self) -> Result<i32, i32> {
        loop {
            if self.pc as usize >= self.program.len() { return Ok(self.accum); }

            let instruction = &mut self.program[self.pc as usize];
            //println!("{:?}", instruction);
            if instruction.executed { return Err(self.accum); }
            match instruction.opcode {
                Opcode::Nop => (),
                Opcode::Acc => self.accum += instruction.operand,
                Opcode::Jmp => self.pc += instruction.operand
            }
            if instruction.opcode != Opcode::Jmp { self.pc += 1 }
            instruction.executed = true;
        }
    }

    fn correct_and_run(&mut self) -> Option<i32> {
        (0..self.program.len()).find_map(|idx| {
            // Try flipping the instruction at idx.
            let flipped = match self.program[idx].opcode {
                Opcode::Nop => Opcode::Jmp,
                Opcode::Jmp => Opcode::Nop,
                _ => None? // bail out
            };

            // Try running the program with the flipped opcode
            let mut interpreter = self.clone();
            interpreter.program[idx].opcode = flipped;
            interpreter.run().ok()
        })
    }
}

pub fn run() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter::parse(
        stdin.lock().lines().map(|line| line.expect("read error"))
    ).expect("parse error");
    let result = interpreter.correct_and_run();
    println!("{:?}", result);
}
