#[derive(Debug)]
struct Mask {
    or: u64,
    floating: Vec<usize>,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            or: 0,
            floating: Vec::new(),
        }
    }
}

impl Mask {
    fn apply<'a>(&'a self, address: u64) -> impl 'a + Iterator<Item = u64> {
        (0..(1 << self.floating.len())).map(move |subset| {
            let mut current_address = address | self.or;
            for (i, floating) in self.floating.iter().enumerate() {
                if subset & (1 << i) == 0 {
                    current_address &= !(1 << floating);
                } else {
                    current_address |= 1 << floating;
                }
            }
            current_address
        })
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
                    'X' => mask.floating.push(offset),
                    '0' => (),
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
                for address in mask.apply(address) {
                    memory.insert(address, value);
                }
            }
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
