use std::fmt;

#[derive(Debug)]
enum Expr {
    Numeral(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

#[derive(Debug)]
enum Tok {
    Numeral(i64),
    Plus,
    Minus,
    Times,
    Divide,
    Lparen,
    Rparen
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Numeral(i) => write!(f, "{}", i),
            &Expr::Plus(ref i, ref j) => write!(f, "{} + {}", i, j),
            &Expr::Minus(ref i, ref j) => write!(f, "{} - {}", i, j),
            &Expr::Times(ref i, ref j) => write!(f, "{} * {}", i, j),
            &Expr::Divide(ref i, ref j) => write!(f, "{} / {}", i, j),
            &Expr::Negate(ref i) => write!(f, "-{}", i),
        }
    }
}

fn eval(expr: Expr) -> i64 {
    match expr {
        Expr::Numeral(i) => i,
        Expr::Plus(i, j) => eval(*i) + eval(*j),
        Expr::Minus(i, j) => eval(*i) - eval(*j),
        Expr::Times(i, j) => eval(*i) * eval(*j),
        Expr::Divide(i, j) => eval(*i) / eval(*j),
        Expr::Negate(i) => eval(*i) * -1,
    }
}

fn char_is_digit(c: &char) -> bool {
    *c == '0' || *c == '1' || *c == '2' || *c == '3' || *c == '4' || *c == '5' || *c == '6' || *c == '7' || *c == '8' || *c == '9'
}

fn lex(line: &str) -> Vec<Tok> {
    let mut toks = vec![];
    let mut iter = line.chars().peekable();

    while let Some(c) = iter.next() {
        if c == ' ' {
            // do nothing
        } else if c == '+' {
            toks.push(Tok::Plus);
        } else if c == '-' {
            toks.push(Tok::Minus);
        } else if c == '*' {
            toks.push(Tok::Times);
        } else if c == '/' {
            toks.push(Tok::Divide);
        } else if c == '(' {
            toks.push(Tok::Lparen);
        } else if c == ')' {
            toks.push(Tok::Rparen);
        } else if char_is_digit(&c) {
            let mut ingesting_numeral = true;
            let mut numeral: Vec<char> = vec![];
            numeral.push(c);

            while ingesting_numeral {
                if let Some(c_digit) = iter.peek() {
                    if char_is_digit(c_digit) {
                        numeral.push(*c_digit);
                    } else {
                        ingesting_numeral = false;
                    }
                } else {
                    ingesting_numeral = false;
                }

                if ingesting_numeral {
                    iter.next();
                }
            }

            let n: String = numeral.iter().cloned().collect();
            if let Ok(i) = n.parse::<i64>() {
                toks.push(Tok::Numeral(i));
            }
        }
    }

    toks
}

fn main() {
    let n1: Expr = Expr::Numeral(5);
    let p: Expr = Expr::Plus(Box::new(Expr::Numeral(5)), Box::new(Expr::Numeral(7)));
    let m: Expr = Expr::Minus(Box::new(Expr::Numeral(5)), Box::new(Expr::Numeral(7)));
    let t: Expr = Expr::Times(Box::new(Expr::Numeral(5)), Box::new(Expr::Numeral(7)));
    let d: Expr = Expr::Divide(Box::new(Expr::Numeral(5)), Box::new(Expr::Numeral(7)));
    let n: Expr = Expr::Negate(Box::new(Expr::Numeral(5)));
    println!("{}", n1);
    println!("{}", p);
    println!("{}", m);
    println!("{}", t);
    println!("{}", d);
    println!("{}", n);

    println!("{}", eval(p));

    println!("{:?}", lex("(1+2) * (5/4)"));
}
