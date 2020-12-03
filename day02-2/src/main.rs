#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl PasswordRule {
    fn parse_stdin() -> Vec<Self> {
        use std::io::BufRead;
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let mut parts1 = line.split(" ");
                let counts = parts1.next().unwrap();
                let ch_with_colon = parts1.next().unwrap();
                let password = parts1.next().unwrap().to_string();
                let mut parts2 = counts.split("-");
                let min = parts2.next().unwrap().parse::<usize>().unwrap();
                let max = parts2.next().unwrap().parse::<usize>().unwrap();
                let ch = ch_with_colon.chars().next().unwrap();
                PasswordRule {
                    min,
                    max,
                    ch,
                    password,
                }
            })
            .collect::<Vec<_>>()
    }

    fn is_valid(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        (chars[self.min - 1] == self.ch) ^ (chars[self.max - 1] == self.ch)
    }
}

fn main() {
    let password_rules = PasswordRule::parse_stdin();
    let count = password_rules.into_iter().filter(|r| r.is_valid()).count();
    println!("{}", count);
}
