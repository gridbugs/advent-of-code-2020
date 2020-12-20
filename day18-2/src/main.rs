#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InfixOp {
    Add,
    Mul,
}

impl InfixOp {
    fn precedence(&self) -> u32 {
        match self {
            Self::Add => 2,
            Self::Mul => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    InfixOp(InfixOp),
    ParenLeft,
    ParenRight,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i64),
    Op(Op),
}

impl Token {
    fn parse_split(s: &str) -> Result<(Self, &str), String> {
        let s = s.trim_start();
        if let Some(r) = s.strip_prefix("(") {
            Ok((Token::Op(Op::ParenLeft), r))
        } else if let Some(r) = s.strip_prefix(")") {
            Ok((Token::Op(Op::ParenRight), r))
        } else if let Some(r) = s.strip_prefix("+") {
            Ok((Token::Op(Op::InfixOp(InfixOp::Add)), r))
        } else if let Some(r) = s.strip_prefix("*") {
            Ok((Token::Op(Op::InfixOp(InfixOp::Mul)), r))
        } else {
            let mut index = 0;
            for ch in s.chars() {
                if !ch.is_digit(10) {
                    break;
                }
                index += ch.len_utf8();
            }
            let (l, r) = s.split_at(index);
            Ok((
                Token::Number(
                    l.parse()
                        .map_err(|_| format!("Can't parse number: {} (full: {})", l, s))?,
                ),
                r,
            ))
        }
    }
}

struct TokenStream<'a> {
    s: &'a str,
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            None
        } else {
            let (token, r) = Token::parse_split(self.s).unwrap();
            self.s = r;
            Some(token)
        }
    }
}

impl<'a> TokenStream<'a> {
    fn new(s: &'a str) -> Self {
        Self { s }
    }
}

fn main() {
    use std::io::BufRead;
    let mut sum = 0;
    for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut op_stack = Vec::new();
        let mut out_stack = Vec::new();
        for token in TokenStream::new(line.as_str()) {
            match token {
                Token::Number(_) => out_stack.push(token),
                Token::Op(Op::InfixOp(op)) => {
                    while let Some(Op::InfixOp(top_op)) = op_stack.last() {
                        if top_op.precedence() <= op.precedence() {
                            break;
                        }
                        out_stack.push(Token::Op(op_stack.pop().unwrap()));
                    }
                    op_stack.push(Op::InfixOp(op));
                }
                Token::Op(Op::ParenLeft) => op_stack.push(Op::ParenLeft),
                Token::Op(Op::ParenRight) => {
                    while let Some(op) = op_stack.pop() {
                        match op {
                            Op::InfixOp(_) => out_stack.push(Token::Op(op)),
                            Op::ParenLeft => {
                                break;
                            }
                            Op::ParenRight => panic!(),
                        }
                    }
                }
            }
        }
        while let Some(op) = op_stack.pop() {
            out_stack.push(Token::Op(op));
        }
        let mut compute_stack = Vec::new();
        for token in out_stack {
            match token {
                Token::Number(n) => compute_stack.push(n),
                Token::Op(Op::InfixOp(op)) => {
                    let r = compute_stack.pop().unwrap();
                    let l = compute_stack.pop().unwrap();
                    match op {
                        InfixOp::Add => compute_stack.push(l + r),
                        InfixOp::Mul => compute_stack.push(l * r),
                    }
                }
                _ => panic!(),
            }
        }
        assert!(compute_stack.len() == 1);
        sum += compute_stack[0];
    }
    println!("{}", sum);
}
