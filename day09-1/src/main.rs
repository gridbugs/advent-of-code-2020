fn main() {
    use std::collections::VecDeque;
    use std::io::BufRead;
    let mut sliding_window = std::io::stdin()
        .lock()
        .lines()
        .take(25)
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<VecDeque<_>>();
    for x in std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
    {
        let mut found_sum = false;
        for w in sliding_window.iter() {
            if let Some(rem) = x.checked_sub(*w) {
                if rem == *w {
                    continue;
                }
                if sliding_window.contains(&rem) {
                    found_sum = true;
                    break;
                }
            }
        }
        if !found_sum {
            println!("{}", x);
            break;
        }
        sliding_window.pop_front().unwrap();
        sliding_window.push_back(x);
    }
}
