#[derive(Debug)]
struct Mask {
    and: u64,
    or: u64,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            and: std::u64::MAX,
            or: 0,
        }
    }
}

impl Mask {
    fn apply(&self, x: u64) -> u64 {
        (x & self.and) | self.or
    }
}

#[derive(Debug)]
enum Instruction {
    SetMask(Mask),
    Write { address: u64, value: u64 },
}

impl std::str::FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mask_str) = s.strip_prefix("mask = ") {
            let mut mask = Mask::default();
            for (i, c) in mask_str.chars().enumerate() {
                let offset = 35 - i;
                match c {
                    'X' => (),
                    '0' => mask.and &= !(1 << offset),
                    '1' => mask.or |= 1 << offset,
                    _ => panic!(),
                }
            }
            return Ok(Self::SetMask(mask));
        }
        if let Some(rest) = s.strip_prefix("mem[") {
            let mut parts = rest.split("] = ");
            let address = parts.next().unwrap().parse::<u64>().unwrap();
            let value = parts.next().unwrap().parse::<u64>().unwrap();
            return Ok(Self::Write { address, value });
        }
        Err(())
    }
}

impl Instruction {
    fn parse_stdin() -> Vec<Self> {
        use std::io::BufRead;
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap().parse::<Instruction>().unwrap())
            .collect()
    }
}

fn main() {
    use std::collections::HashMap;
    let mut mask = Mask::default();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let instructions = Instruction::parse_stdin();
    for instruction in instructions {
        match instruction {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::Write { address, value } => {
                memory.insert(address, mask.apply(value));
            }
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
