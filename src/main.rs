use std::fmt;
use std::io;
use std::io::Write;

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
    Rparen,
    Eof,
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

    toks.push(Tok::Eof);
    toks
}


/*
    recursive descent with operator precedence climbing
*/
fn parse(toks: &Vec<Tok>) -> Result<Expr, &'static str> {
    if toks.len() == 0 {
        return Err("Can't parse; no tokens.");
    }

    let mut current_token: usize = 0;

    parse_expr(toks, &mut current_token)
}

fn parse_expr(toks: &Vec<Tok>, current_token: &mut usize) -> Result<Expr, &'static str> {
    let lhs = try!(parse_primary(toks, current_token));
    parse_binary_ops_rhs(toks, current_token, 0, lhs)
}

fn parse_primary(toks: &Vec<Tok>, current_token: &mut usize) -> Result<Expr, &'static str> {
    let tok = try!(get_token(toks, current_token));
    match *tok {
        Tok::Minus => parse_negate(toks, current_token),
        Tok::Numeral(_) => parse_numeral(toks, current_token),
        Tok::Lparen => parse_paren(toks, current_token),
        _ => Err("Only negation, parens, or numerals are primary expressions.")
    }
}

fn parse_binary_ops_rhs(toks: &Vec<Tok>, current_token: &mut usize, expr_precedence: i8, mut left_expr: Expr) -> Result<Expr, &'static str> {
    loop {
        let tok = try!(get_token(toks, current_token));
        let tok_precedence = op_precedence(tok);

        if tok_precedence < expr_precedence {
            return Ok(left_expr)
        } else {
            // since non binary operators have negative precedence, tok is a binary op here.
            eat_token(current_token); // eat binop
            let mut right_expr = try!(parse_primary(toks, current_token));

            // If BinOp binds less tightly with RHS than the operator after RHS, let
            // the pending operator take RHS as its LHS.

            let next_op = try!(get_token(toks, current_token));
            let next_prec = op_precedence(next_op);
            if tok_precedence < next_prec {
                right_expr = try!(parse_binary_ops_rhs(toks, current_token, expr_precedence + 1, right_expr))
            }

            left_expr = binary_op_expr(tok, left_expr, right_expr)
        }
    }
}

fn binary_op_expr(binary_op: &Tok, lhs: Expr, rhs: Expr) -> Expr {
    match *binary_op {
        Tok::Plus => Expr::Plus(Box::new(lhs), Box::new(rhs)),
        Tok::Minus => Expr::Minus(Box::new(lhs), Box::new(rhs)),
        Tok::Times => Expr::Times(Box::new(lhs), Box::new(rhs)),
        Tok::Divide => Expr::Divide(Box::new(lhs), Box::new(rhs)),
        _ => unreachable!()
    }
}

// Expects toks[current_token] to be a Tok::Numeral, panics otherwise
fn parse_numeral(toks: &Vec<Tok>, current_token: &mut usize) -> Result<Expr, &'static str> {
    let tok = try!(get_token(toks, current_token));

    match *tok {
        Tok::Numeral(i) => {
            eat_token(current_token); // eat numeral
            Ok(Expr::Numeral(i))
        },
        _ => unreachable!()
    }
}

fn parse_paren(toks: &Vec<Tok>, current_token: &mut usize) -> Result<Expr, &'static str> {
    eat_token(current_token); // eat left paren

    let expr = try!(parse_expr(toks, current_token));

    let tok = try!(get_token(toks, current_token));
    match *tok {
        Tok::Rparen => {
            eat_token(current_token); // eat right paren
            Ok(expr)
        },
        _ => Err("Unmatched left parenthesis."),
    }
}

fn parse_negate(toks: &Vec<Tok>, current_token: &mut usize) -> Result<Expr, &'static str> {
    eat_token(current_token); // eat minus

    let tok = try!(get_token(toks, current_token));
    match *tok {
        Tok::Numeral(i) => {
            eat_token(current_token); // eat numeral
            Ok(Expr::Negate(Box::new(Expr::Numeral(i))))
        },
        _ => Err("Can't negate anything but a numeral.")
    }
}

fn op_precedence(tok: &Tok) -> i8 {
    match *tok {
        Tok::Numeral(_) => -1,
        Tok::Plus => 10,
        Tok::Minus => 10,
        Tok::Times => 20,
        Tok::Divide => 20,
        Tok::Lparen => -1,
        Tok::Rparen => -1,
        Tok::Eof => -1,
    }
}

fn eat_token(current_token: &mut usize) {
    *current_token += 1;
}

fn get_token<'a, 'b>(toks: &'a Vec<Tok>, current_token: &'b mut usize) -> Result<&'a Tok, &'static str> {
    if let Some(tok) = toks.get(*current_token) {
        Ok(tok)
    } else {
        Err("Attempted to get token but there are no more.")
    }
}

fn main() {
    println!("Welcome to calc!");
    loop {
        print!("     > ");
        if let Err(x) = io::stdout().flush() {
            println!("could not flush: {}", x);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Could not read line.");
        let lexed = lex(&input);
        match parse(&lexed) {
            Ok(parsed) => {
                println!("{}", eval(parsed));
            },
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }
}
