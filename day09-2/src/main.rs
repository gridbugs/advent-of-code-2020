fn read_all_stdin() -> Vec<u64> {
    use std::io::BufRead;
    std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn first_number_without_sum(xs: &[u64]) -> Option<u64> {
    use std::collections::VecDeque;
    const WIDTH: usize = 25;
    let mut sliding_window = xs.iter().cloned().take(WIDTH).collect::<VecDeque<_>>();
    for x in xs[WIDTH..].iter().cloned() {
        let mut found_sum = false;
        for w in sliding_window.iter().cloned() {
            if let Some(rem) = x.checked_sub(w) {
                if rem == w {
                    continue;
                }
                if sliding_window.contains(&rem) {
                    found_sum = true;
                    break;
                }
            }
        }
        if !found_sum {
            return Some(x);
        }
        sliding_window.pop_front().unwrap();
        sliding_window.push_back(x);
    }
    None
}

fn find_sum(x: u64, xs: &[u64]) -> u64 {
    let mut lo = 0;
    let mut hi = 0;
    let mut sum = 0;
    while hi <= xs.len() {
        while sum > x {
            sum -= xs[lo];
            lo += 1;
        }
        assert!(lo <= hi);
        while sum < x {
            sum += xs[hi];
            hi += 1;
        }
        if sum == x {
            let range = &xs[lo..hi];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            return min + max;
        }
    }
    panic!()
}

fn main() {
    let xs = read_all_stdin();
    let x = first_number_without_sum(&xs).unwrap();
    let result = find_sum(x, &xs);
    println!("{}", result);
}
