#[derive(Debug)]
enum Expr {
    Numeral(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

fn main() {
    let n1: Expr = Expr::Numeral(5);
    println!("{:?}", n1);
}
