use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Rule {
    fn is_in_any_range(&self, val: u32) -> bool {
        for range in &self.ranges {
            if range.contains(&val) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    fn parse_stdin() -> Self {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        let mut read_line = move || {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            buf.trim_end_matches("\n").to_string()
        };
        let mut rules = Vec::new();
        loop {
            let line = read_line();
            if line.is_empty() {
                break;
            }
            let mut parts0 = line.split(": ");
            let name = parts0.next().unwrap().to_string();
            let ranges_s = parts0.next().unwrap();
            let mut parts1 = ranges_s.split(" or ");
            let range_l_s = parts1.next().unwrap();
            let range_r_s = parts1.next().unwrap();
            fn parse_range(s: &str) -> RangeInclusive<u32> {
                let mut parts = s.split("-");
                let l = parts.next().unwrap().parse::<u32>().unwrap();
                let r = parts.next().unwrap().parse::<u32>().unwrap();
                l..=r
            }
            let ranges = vec![parse_range(range_l_s), parse_range(range_r_s)];
            rules.push(Rule { name, ranges });
        }
        assert!(read_line() == "your ticket:");
        let my_ticket = Ticket {
            fields: read_line()
                .split(",")
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        };
        assert!(read_line() == "");
        assert!(read_line() == "nearby tickets:");
        let mut nearby_tickets = Vec::new();
        loop {
            let line = read_line();
            if line.is_empty() {
                break;
            }
            let fields = line
                .split(",")
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            nearby_tickets.push(Ticket { fields });
        }
        Self {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

fn main() {
    let input = Input::parse_stdin();
    let mut error_rate = 0;
    for ticket in &input.nearby_tickets {
        'field: for &field in &ticket.fields {
            for rule in &input.rules {
                if rule.is_in_any_range(field) {
                    continue 'field;
                }
            }
            error_rate += field;
        }
    }
    println!("{}", error_rate);
}
