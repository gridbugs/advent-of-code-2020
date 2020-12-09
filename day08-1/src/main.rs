#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Jmp,
    Acc,
    Nop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    typ: InstructionType,
    arg: i32,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let typ = match parts.next().unwrap() {
            "nop" => InstructionType::Nop,
            "acc" => InstructionType::Acc,
            "jmp" => InstructionType::Jmp,
            _ => panic!(),
        };
        let arg = parts.next().unwrap().parse::<i32>().unwrap();
        Self { typ, arg }
    }
    fn parse_stdin() -> Vec<Self> {
        use std::io::BufRead;
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| Self::parse(l.unwrap().as_str()))
            .collect()
    }
}

#[derive(Default)]
struct Machine {
    acc: i32,
    pc: i32,
}

impl Machine {
    fn eval_instruction(&mut self, instruction: Instruction) {
        match instruction.typ {
            InstructionType::Nop => (),
            InstructionType::Acc => self.acc += instruction.arg,
            InstructionType::Jmp => {
                self.pc += instruction.arg;
                return;
            }
        }
        self.pc += 1;
    }
}

fn main() {
    use std::collections::HashSet;
    let program = Instruction::parse_stdin();
    let mut machine = Machine::default();
    let mut seen_pcs = HashSet::new();
    loop {
        if !seen_pcs.insert(machine.pc) {
            break;
        }
        let instruction = program[machine.pc as usize];
        machine.eval_instruction(instruction);
    }
    println!("{}", machine.acc);
}
