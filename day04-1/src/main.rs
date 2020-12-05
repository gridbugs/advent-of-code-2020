use std::collections::HashMap;

#[derive(Debug)]
struct PassportCandidate {
    fields: HashMap<String, String>,
}

impl PassportCandidate {
    fn is_valid(&self) -> bool {
        let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_fields.iter().all(|&f| self.fields.contains_key(f))
    }
}

impl PassportCandidate {
    fn parse_stdin() -> Vec<Self> {
        use std::io::Read;
        let mut string = String::new();
        std::io::stdin().lock().read_to_string(&mut string).unwrap();
        string
            .split("\n\n")
            .map(|s| {
                let fields = s
                    .split_whitespace()
                    .map(|f| {
                        let mut key_value = f.split(":");
                        let key = key_value.next().unwrap();
                        let value = key_value.next().unwrap();
                        (key.to_string(), value.to_string())
                    })
                    .collect::<HashMap<String, String>>();
                PassportCandidate { fields }
            })
            .collect()
    }
}

fn main() {
    let passport_candidates = PassportCandidate::parse_stdin();
    println!(
        "{}",
        passport_candidates.iter().filter(|p| p.is_valid()).count()
    );
}
