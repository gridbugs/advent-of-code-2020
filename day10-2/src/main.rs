fn main() {
    use std::io::BufRead;
    let mut input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();
    let mut counts = vec![0u64; input.len()];
    counts[0] = 1;
    for (i, x) in input.iter().enumerate().skip(1) {
        let mut count = counts[i - 1];
        if i >= 2 {
            if x - input[i - 2] <= 3 {
                count += counts[i - 2];
            }
        }
        if i >= 3 {
            if x - input[i - 3] <= 3 {
                count += counts[i - 3];
            }
        }
        counts[i] = count;
    }
    println!("{}", counts.last().unwrap());
}
