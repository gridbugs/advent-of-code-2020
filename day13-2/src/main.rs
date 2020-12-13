#[derive(Debug, Clone, Copy)]
struct Term {
    multiply: u64,
    subtract: u64,
}

impl Term {
    fn parse_stdin() -> Vec<Self> {
        use std::io::BufRead;
        let lines = std::io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>();
        let bus_ids = lines[1]
            .split(",")
            .map(|i| {
                if i == "x" {
                    None
                } else {
                    Some(i.parse::<u64>().unwrap())
                }
            })
            .collect::<Vec<_>>();
        let mut offset = 0;
        let mut ret = Vec::new();
        for maybe_id in bus_ids {
            if let Some(id) = maybe_id {
                ret.push(Term {
                    multiply: id,
                    subtract: offset,
                });
            }
            offset += 1;
        }
        ret
    }

    fn merge(&self, other: &Self) -> Self {
        for i in 1..=other.multiply {
            let candidate = (self.multiply * i) - self.subtract;
            if (candidate + other.subtract) % other.multiply == 0 {
                let multiply = self.multiply * other.multiply;
                let subtract = multiply - candidate;
                return Self { multiply, subtract };
            }
        }
        panic!()
    }

    fn result(&self) -> u64 {
        self.multiply - self.subtract
    }
}

fn main() {
    let terms = Term::parse_stdin();
    let mut acc = terms[0];
    for &t in &terms[1..] {
        acc = acc.merge(&t);
    }
    println!("{:?}", acc.result());
}
