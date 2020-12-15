fn main() {
    let input = {
        use std::io::Read;
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        input.truncate(input.len() - 1);
        input
            .split(",")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    };
    use std::collections::{hash_map::Entry, HashMap};
    let mut number_to_turn = input[0..input.len() - 1]
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();
    let mut prev = *input.last().unwrap();
    for i in input.len() - 1..2019 {
        match number_to_turn.entry(prev) {
            Entry::Occupied(mut o) => {
                let last_position = *o.get();
                prev = (i - last_position) as u64;
                o.insert(i);
            }
            Entry::Vacant(v) => {
                v.insert(i);
                prev = 0;
            }
        }
    }
    println!("{}", prev);
}
