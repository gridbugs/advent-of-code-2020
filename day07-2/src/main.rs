#[derive(Debug)]
struct Contents {
    count: u32,
    bag: String,
}

fn main() {
    use std::collections::HashMap;
    use std::io::BufRead;
    let rules = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut parts0 = line.split(" contain ");
            let lhs = parts0.next().unwrap().strip_suffix(" bags").unwrap();
            let rhs = parts0.next().unwrap().strip_suffix(".").unwrap();
            let contents = if rhs == "no other bags" {
                Vec::new()
            } else {
                rhs.split(", ")
                    .map(|s| {
                        let mut parts = s
                            .strip_suffix(" bag")
                            .or_else(|| s.strip_suffix(" bags"))
                            .unwrap()
                            .split_whitespace();
                        let count = parts.next().unwrap().parse::<u32>().unwrap();
                        let bag = parts.collect::<Vec<_>>().join(" ");
                        Contents { count, bag }
                    })
                    .collect()
            };
            (lhs.to_string(), contents)
        })
        .collect::<HashMap<_, _>>();
    let root = "shiny gold";
    let mut to_visit = vec![(root, 1)];
    let mut total = 0;
    while let Some((bag, n)) = to_visit.pop() {
        for b in &rules[bag] {
            let next_n = b.count * n;
            total += next_n;
            to_visit.push((b.bag.as_str(), next_n));
        }
    }
    println!("{}", total);
}
