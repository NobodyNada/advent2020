use std::io::{self, BufRead};

struct Interpreter {
    program: Vec<Instruction>,
    pc: i32,
    accum: i32
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: i32,

    /// Whether or not this instruction has been executed.
    executed: bool
}

#[derive(Eq, PartialEq, Debug)]
enum Opcode {
    Nop,
    Acc,
    Jmp
}

impl Instruction { fn parse(input: &str) -> Option<Self> {
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
    /// Parses the input into a program. Returns the bad instruction on failure.
    fn parse(input: impl Iterator<Item=String>) -> Result<Self, String> {
        let program = input.map(|line| Instruction::parse(&line).ok_or(line)).collect::<Result<Vec<Instruction>,_>>()?;
        Ok(Interpreter { program, pc: 0, accum: 0 })
    }

    fn run_until_infinite_loop(&mut self) -> i32 {
        loop {
            let instruction = &mut self.program[self.pc as usize];
            //println!("{:?}", instruction);
            if instruction.executed { return self.accum; }
            match instruction.opcode {
                Opcode::Nop => (),
                Opcode::Acc => self.accum += instruction.operand,
                Opcode::Jmp => self.pc += instruction.operand
            }
            if instruction.opcode != Opcode::Jmp { self.pc += 1 }
            instruction.executed = true;
        }
    }
}


pub fn run() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter::parse(
        stdin.lock().lines().map(|line| line.expect("read error"))
    ).expect("parse error");
    let result = interpreter.run_until_infinite_loop();
    println!("{}", result);
}
