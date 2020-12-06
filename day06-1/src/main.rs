fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let count: usize = input
        .split("\n\n")
        .map(|g| {
            use std::collections::HashSet;
            let mut s = g.chars().collect::<HashSet<_>>();
            s.remove(&'\n');
            s.len()
        })
        .sum();
    println!("{}", count);
}
