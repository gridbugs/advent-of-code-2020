use std::collections::HashSet;

fn main() {
    use std::io::BufRead;
    let ids = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut digit = 512;
            let mut id = 0;
            line.chars().for_each(|c| {
                if c == 'B' || c == 'R' {
                    id += digit;
                }
                digit /= 2;
            });
            id
        })
        .collect::<Vec<_>>();
    let min_id = *ids.iter().min().unwrap();
    let max_id = *ids.iter().max().unwrap();
    let mut unseen = (min_id..=max_id).into_iter().collect::<HashSet<_>>();
    for id in ids {
        unseen.remove(&id);
    }
    assert_eq!(unseen.len(), 1);
    println!("{}", unseen.iter().next().unwrap());
}
