use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Literal(char),
    AnyOfSeq(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, Rule>,
}

impl Rules {
    fn check(&self, s: &str) -> bool {
        #[derive(Clone, Debug)]
        struct Stack<'a> {
            indices: Vec<usize>,
            string: &'a str,
        }
        let mut stacks = vec![Stack {
            indices: vec![0],
            string: s,
        }];
        while let Some(mut stack) = stacks.pop() {
            if let Some(rule_index) = stack.indices.pop() {
                if stack.string.is_empty() {
                    continue;
                }
                match &self.rules[&rule_index] {
                    Rule::Literal(ch) => {
                        if let Some(first) = stack.string.chars().next() {
                            if *ch == first {
                                let rest = &stack.string[first.len_utf8()..];
                                stack.string = rest;
                                stacks.push(stack.clone());
                            }
                        }
                    }
                    Rule::AnyOfSeq(any_of_seq) => {
                        for seq in any_of_seq.iter().rev() {
                            let mut new_stack = stack.clone();
                            for &i in seq.iter().rev() {
                                new_stack.indices.push(i);
                            }
                            stacks.push(new_stack);
                        }
                    }
                }
            }
            if stack.indices.is_empty() && stack.string.is_empty() {
                return true;
            }
        }
        false
    }
}

fn parse_stdin() -> (Rules, Vec<String>) {
    let lines = std::io::stdin();
    let mut buf = String::new();
    let mut rules = HashMap::new();
    loop {
        buf.clear();
        lines.read_line(&mut buf).unwrap();
        let line = buf.trim_end();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let index = parts.next().unwrap().parse::<usize>().unwrap();
        let rhs = parts.next().unwrap();
        if let Some(rest) = rhs.strip_prefix("\"") {
            rules.insert(index, Rule::Literal(rest.chars().next().unwrap()));
        } else {
            let any_of_seq = rhs
                .split(" | ")
                .map(|s| {
                    s.split(" ")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            rules.insert(index, Rule::AnyOfSeq(any_of_seq));
        }
    }
    let mut strings = Vec::new();
    loop {
        buf.clear();
        lines.read_line(&mut buf).unwrap();
        let line = buf.trim_end();
        if line.is_empty() {
            break;
        }
        strings.push(line.to_string());
    }
    (Rules { rules }, strings)
}

fn main() {
    let (rules, strings) = parse_stdin();
    let mut count = 0;
    for string in strings {
        if rules.check(&string) {
            println!("{}", string);
            count += 1;
        }
    }
    println!("{}", count);
}
