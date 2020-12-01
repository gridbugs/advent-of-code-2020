fn main() {
    use std::io::BufRead;
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for (i, a) in entries.iter().enumerate() {
        for (j, b) in entries[i..].iter().enumerate() {
            for c in &entries[j..] {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
