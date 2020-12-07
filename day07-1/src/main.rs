fn main() {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::io::BufRead;
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    std::io::stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();
        let mut parts0 = line.split(" contain ");
        let lhs = parts0.next().unwrap().strip_suffix(" bags").unwrap();
        let rhs = parts0.next().unwrap().strip_suffix(".").unwrap();
        rhs.split(", ")
            .filter_map(|s| {
                if s == "no other bags" {
                    None
                } else {
                    let mut parts = s
                        .strip_suffix(" bag")
                        .or_else(|| s.strip_suffix(" bags"))
                        .unwrap()
                        .split_whitespace();
                    parts.next().unwrap();
                    Some(parts.collect::<Vec<_>>().join(" "))
                }
            })
            .for_each(|b| {
                rules
                    .entry(b)
                    .or_insert_with(Vec::new)
                    .push(lhs.to_string());
            });
    });
    let root = "shiny gold";
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(root);
    while let Some(current) = to_visit.pop_front() {
        if let Some(bags) = rules.get(current) {
            for b in bags {
                if seen.insert(b) {
                    to_visit.push_back(b);
                }
            }
        }
    }
    println!("{}", seen.len());
}
