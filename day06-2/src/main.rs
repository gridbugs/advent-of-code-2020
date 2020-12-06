fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let count: usize = input
        .split("\n\n")
        .map(|g| {
            use std::collections::HashSet;
            let mut rows = g.split("\n");
            let mut common = rows.next().unwrap().chars().collect::<HashSet<_>>();
            for row in rows {
                if row.is_empty() {
                    continue;
                }
                common = common
                    .intersection(&row.chars().collect())
                    .cloned()
                    .collect();
            }
            common.len()
        })
        .sum();
    println!("{}", count);
}
