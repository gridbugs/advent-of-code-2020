use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u64>>,
}

impl Rule {
    fn is_in_any_range(&self, val: u64) -> bool {
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
    fields: Vec<u64>,
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
            fn parse_range(s: &str) -> RangeInclusive<u64> {
                let mut parts = s.split("-");
                let l = parts.next().unwrap().parse::<u64>().unwrap();
                let r = parts.next().unwrap().parse::<u64>().unwrap();
                l..=r
            }
            let ranges = vec![parse_range(range_l_s), parse_range(range_r_s)];
            rules.push(Rule { name, ranges });
        }
        assert!(read_line() == "your ticket:");
        let my_ticket = Ticket {
            fields: read_line()
                .split(",")
                .map(|f| f.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        };
        assert!(read_line() == "");
        assert!(read_line() == "nearby tickets:");
        let mut nearby_ticket_candidates = Vec::new();
        loop {
            let line = read_line();
            if line.is_empty() {
                break;
            }
            let fields = line
                .split(",")
                .map(|f| f.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            nearby_ticket_candidates.push(Ticket { fields });
        }
        let nearby_tickets = nearby_ticket_candidates
            .into_iter()
            .filter(|ticket| {
                for &field in &ticket.fields {
                    let mut valid = false;
                    for rule in &rules {
                        if rule.is_in_any_range(field) {
                            valid = true;
                        }
                    }
                    if !valid {
                        return false;
                    }
                }
                true
            })
            .collect();
        Self {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

fn main() {
    use std::collections::HashSet;
    let input = Input::parse_stdin();
    let mut candidate_fields = input
        .my_ticket
        .fields
        .iter()
        .enumerate()
        .map(|(i, _)| {
            input
                .rules
                .iter()
                .filter_map(|rule| {
                    if input
                        .nearby_tickets
                        .iter()
                        .all(|ticket| rule.is_in_any_range(ticket.fields[i]))
                    {
                        Some(rule.name.clone())
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut solved_field_names = candidate_fields
        .iter()
        .filter_map(|fields| {
            if fields.len() == 1 {
                Some(fields.iter().next().unwrap().clone())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    while solved_field_names.len() < candidate_fields.len() {
        for candidate_rule_names in candidate_fields.iter_mut() {
            if candidate_rule_names.len() == 1 {
                continue;
            }
            for solved in solved_field_names.iter() {
                candidate_rule_names.remove(solved);
            }
            if candidate_rule_names.len() == 1 {
                solved_field_names.insert(candidate_rule_names.iter().next().unwrap().clone());
            }
        }
    }
    let mut result = 1;
    for (name, field) in candidate_fields.iter().zip(input.my_ticket.fields.iter()) {
        if name.iter().next().unwrap().starts_with("departure ") {
            result *= field;
        }
    }
    println!("{}", result);
}
