fn main() {
    use std::io::BufRead;
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let earliest_timestamp = lines[0].parse::<u32>().unwrap();
    let bus_ids = lines[1]
        .split(",")
        .filter(|i| i != &"x")
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let (best_id, pickup_time) = bus_ids
        .iter()
        .cloned()
        .map(|i| (i, i * (1 + (earliest_timestamp - 1) / i)))
        .min_by_key(|(_, e)| *e)
        .unwrap();
    println!("{}", (pickup_time - earliest_timestamp) * best_id);
}
