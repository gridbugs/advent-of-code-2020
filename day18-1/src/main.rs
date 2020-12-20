#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Equation {
    lhs: Expr,
    op: Op,
    rhs: Expr,
}

#[derive(Debug)]
enum Expr {
    Atom(i64),
    Equation(Box<Equation>),
}

impl Expr {
    fn parse_atom(s: &str) -> Result<(i64, &str), ()> {
        let mut index = 0;
        for ch in s.chars() {
            if !ch.is_digit(10) {
                break;
            }
            index += 1;
        }
        let (left, right) = s.split_at(index);
        Ok((left.parse().map_err(|_| ())?, right))
    }

    fn parse_op(s: &str) -> Result<(Op, &str), ()> {
        if let Some(rest) = s.strip_prefix("+") {
            Ok((Op::Add, rest))
        } else if let Some(rest) = s.strip_prefix("*") {
            Ok((Op::Mul, rest))
        } else {
            Err(())
        }
    }

    fn parse_parenthesised(s: &str) -> Result<(Expr, &str), ()> {
        if let Some(rest) = s.strip_prefix("(") {
            let mut count = 1;
            for (i, ch) in rest.char_indices() {
                match ch {
                    '(' => count += 1,
                    ')' => {
                        count -= 1;
                        if count == 0 {
                            let (inner, outer) = rest.split_at(i);
                            let expr = Self::parse(inner)?;
                            return Ok((expr, &outer[1..]));
                        }
                    }
                    _ => (),
                }
            }
            Err(())
        } else {
            Err(())
        }
    }

    fn parse_expr(s: &str) -> Result<(Self, &str), ()> {
        let first = s.chars().next().ok_or(())?;
        if first == '(' {
            Self::parse_parenthesised(s)
        } else if first.is_digit(10) {
            let (a, r) = Self::parse_atom(s)?;
            Ok((Expr::Atom(a), r))
        } else {
            Err(())
        }
    }

    fn parse_step(lhs: Expr, s: &str) -> Result<(Self, &str), ()> {
        let s = s.trim_start();
        if s.starts_with("+") || s.starts_with("*") {
            let (op, r) = Self::parse_op(s)?;
            let r = r.trim_start();
            let (rhs, r) = Self::parse_expr(r)?;
            let equation = Equation { lhs, rhs, op };
            Ok((Expr::Equation(Box::new(equation)), r))
        } else if s.is_empty() {
            Ok((lhs, ""))
        } else {
            Err(())
        }
    }

    fn parse(s: &str) -> Result<Self, ()> {
        let s = s.trim_start();
        let (mut e, mut r) = Self::parse_expr(s)?;
        loop {
            let (e1, r1) = Self::parse_step(e, r)?;
            e = e1;
            r = r1.trim_start();
            if r.is_empty() {
                return Ok(e);
            }
        }
    }

    fn eval(&self) -> i64 {
        match self {
            Self::Atom(a) => *a,
            Self::Equation(e) => match e.op {
                Op::Add => e.lhs.eval() + e.rhs.eval(),
                Op::Mul => e.lhs.eval() * e.rhs.eval(),
            },
        }
    }
}

fn main() {
    use std::io::BufRead;
    let mut sum = 0;
    for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
        println!("{}", line);
        if let Ok(e) = Expr::parse(line.as_str()) {
            //println!("{:?}", e);
            println!("{}", e.eval());
            sum += e.eval();
        } else {
            panic!();
        }
    }
    println!("{}", sum);
}
