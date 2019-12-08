use Mode::*;
use IntCodeError::*;
use std::iter::repeat;
use std::convert::TryFrom;

static INPUT: &str = include_str!("../inputs/day05");

#[derive(Copy, Clone, Debug)]
enum IntCodeError {
    NoInstructions,
    TooFewParameters,
    InvalidMode(i32),
    InvalidOpCode(i32),
    MalformedInstruction,
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add,
    Mul,
    Read,
    Write,
    JumpTrue,
    JumpFalse,
    LessThan,
    Eq,
    Halt,
}

impl TryFrom<i32> for OpCode {
    type Error = IntCodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use OpCode::*;
        match value % 100 {
             1 => Ok(Add),
             2 => Ok(Mul),
             3 => Ok(Read),
             4 => Ok(Write),
             5 => Ok(JumpTrue),
             6 => Ok(JumpFalse),
             7 => Ok(LessThan),
             8 => Ok(Eq),
            99 => Ok(Halt),
            op => Err(InvalidOpCode(op)),
        }
    }
}

impl OpCode {
    fn len(self) -> usize {
        self.parameters() + 1
    }

    fn parameters(self) -> usize {
        use OpCode::*;
        match self {
            Add | Mul | LessThan | Eq => 3,
            JumpTrue | JumpFalse => 2,
            Read | Write => 1,
            Halt => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Position
    }
}

fn specified_modes(modes_and_opcode: i32) -> impl Iterator<Item = Result<Mode, IntCodeError>> {
    let modes = modes_and_opcode / 100;
    let num_modes = (modes as f32).log10() as u32 + 1;
    (0..num_modes)
        .map(move |exp| modes / 10i32.pow(exp) % 10)
        .map(Mode::try_from)
}

impl TryFrom<i32> for Mode {
    type Error = IntCodeError;

    fn try_from(mode: i32) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(Position),
            1 => Ok(Immediate),
            m => Err(InvalidMode(m)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Parameter {
    mode: Mode,
    value: i32,
}

impl From<(Mode, &i32)> for Parameter {
    fn from((mode, &value): (Mode, &i32)) -> Self {
        Parameter { mode, value }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    opcode: OpCode,
    parameters: [Option<Parameter>; 3],
}

impl Instruction {
    fn len(&self) -> usize {
        self.opcode.len()
    }
}

impl TryFrom<&[i32]> for Instruction {
    type Error = IntCodeError;

    fn try_from(instructions: &[i32]) -> Result<Self, Self::Error> {
        if instructions.is_empty() {
            return Err(NoInstructions);
        }

        let opcode = OpCode::try_from(instructions[0])?;
        if opcode.len() > instructions.len() {
            return Err(TooFewParameters);
        }

        let maybe_modes = specified_modes(instructions[0]).chain(repeat(Ok(Mode::default())));
        let mut modes = Vec::with_capacity(opcode.parameters());

        for maybe_mode in maybe_modes.take(opcode.parameters()) {
            modes.push(maybe_mode?);
        }

        let mut parameters = [None; 3];

        for i in 0..opcode.parameters() {
            parameters[i] = Some(Parameter {
                mode: modes[i],
                value: instructions[i + 1],
            });
        }

        Ok(Instruction { opcode, parameters })
    }
}

fn load_instructions() -> Vec<i32> {
    INPUT
        .trim()
        .split(',')
        .map(|n| n.parse().expect("Failed to load instructions"))
        .collect()
}

fn next_instruction(ip: usize, instructions: &[i32]) -> Result<Instruction, IntCodeError> {
    Instruction::try_from(&instructions[ip..])
}

trait IntCodeTape {
    fn value_from(&self, param: &Parameter) -> i32;
    fn set_value(&mut self, param: &Parameter, value: i32);
}

impl IntCodeTape for [i32] {
    fn value_from(&self, param: &Parameter) -> i32 {
       match param.mode {
           Position  => self[param.value as usize], 
           Immediate => param.value,
       } 
    }
    
    fn set_value(&mut self, param: &Parameter, value: i32) {
        if let Position = param.mode {
            self[param.value as usize] = value; 
        }
    }
}

fn run(tape: &mut [i32], input: i32) -> Result<i32, IntCodeError> {
    use OpCode::*;
    let mut ip = 0;
    loop {
        let mut instruction = next_instruction(ip, tape)?;
        match &mut instruction {
            Instruction { opcode: Add, parameters: [Some(lhs), Some(rhs), Some(dest)] } => {
                tape.set_value(dest, tape.value_from(lhs) + tape.value_from(rhs));
            }
            Instruction { opcode: Mul, parameters: [Some(lhs), Some(rhs), Some(dest)] } => {
                tape.set_value(dest, tape.value_from(lhs) * tape.value_from(rhs));
            }
            Instruction { opcode: Read, parameters: [Some(index), _, _] } => {
                tape.set_value(index, input);
            }
            Instruction { opcode: Write, parameters: [Some(index), _, _] } => {
                println!("Output: {}", tape.value_from(index));
            }
            Instruction { opcode: JumpTrue, parameters: [Some(predicate), Some(index), _] } => {
                if tape.value_from(predicate) != 0 {
                    ip = tape.value_from(index) as usize;
                    continue;
                }
            }
            Instruction { opcode: JumpFalse, parameters: [Some(predicate), Some(index), _] } => {
                if tape.value_from(predicate) == 0 {
                    ip = tape.value_from(index) as usize;
                    continue;
                }
            }
            Instruction { opcode: LessThan, parameters: [Some(lhs), Some(rhs), Some(dest)] } => {
                let value = if tape.value_from(lhs) < tape.value_from(rhs) { 1 } else { 0 };
                tape.set_value(dest, value); 
            }
            Instruction { opcode: Eq, parameters: [Some(lhs), Some(rhs), Some(dest)] } => {
                let value = if tape.value_from(lhs) == tape.value_from(rhs) { 1 } else { 0 };
                tape.set_value(dest, value);
            }
            Instruction { opcode: Halt, .. } => {
                break;
            }
            _ => return Err(MalformedInstruction),
        }
        ip += instruction.len();
    }
    Ok(tape[0])
}

pub fn part1() {
    println!("Day05 Part1:");
    if run(&mut load_instructions(), 1).is_err() {
        println!("Failed");
    }
}

pub fn part2() {
    println!("Day05 Part2:");
    if run(&mut load_instructions(), 5).is_err() {
        println!("Failed");
    }
}
