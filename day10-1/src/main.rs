fn main() {
    use std::io::BufRead;
    let mut input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    input.sort();
    let gaps1 = input.windows(2).filter(|w| w[1] - w[0] == 1).count() + (input[0] == 1) as usize;
    let gaps3 =
        input.windows(2).filter(|w| w[1] - w[0] == 3).count() + (input[0] == 3) as usize + 1;
    println!("{}", gaps1 * gaps3);
}
