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
}
