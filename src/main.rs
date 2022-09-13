extern crate core;

use core::panicking::panic;
use std::iter::Peekable;
use std::slice::{Iter};

enum Syntax {
    Atom(String),
    Not(Box<Proposition>),
    Times(Box<Proposition>, Box<Proposition>),
    Plus(Box<Proposition>, Box<Proposition>),
    With(Box<Proposition>, Box<Proposition>),
    Par(Box<Proposition>, Box<Proposition>),
    OfCourse(Box<Proposition>),
    WhyNot(Box<Proposition>),
    One,
    Zero,
    Top,
    Bottom,
    Gamma,
    Delta,
    Entailment {
        left: Box<Proposition>,
        right: Box<Proposition>,
    },
}

fn tokenize(&string: &str) -> Vec<String> {
    string
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string()).collect()
}

fn parse(tokens: &mut Peekable<Iter<String>>) -> Syntax {
    let token = tokens.next().unwrap();
    match token.as_str() {
        "(" => {
            let token = tokens.next().unwrap();
            match token.as_str() {
                "not" => {
                    let proposition = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::Not(Box::new(proposition))
                }
                "times" => {
                    let left = parse(tokens);
                    let right = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::Times(Box::new(left), Box::new(right))
                }
                "plus" => {
                    let left = parse(tokens);
                    let right = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::Plus(Box::new(left), Box::new(right))
                }
                "with" => {
                    let left = parse(tokens);
                    let right = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::With(Box::new(left), Box::new(right))
                }
                "par" => {
                    let left = parse(tokens);
                    let right = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::Par(Box::new(left), Box::new(right))
                }
                "of-course" => {
                    let proposition = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::OfCourse(Box::new(proposition))
                }
                "why-not" => {
                    let proposition = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::WhyNot(Box::new(proposition))
                }
                "entails" => {
                    let left = parse(tokens);
                    let right = parse(tokens);
                    tokens.next(); // Skip ")"
                    Syntax::Entailment {
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                }
                _ => panic!("Unexpected token: {}", token),
            }
        }
        "1" => {
            Syntax::One
        }
        "0" => {
            Syntax::Zero
        }
        "T" => {
            Syntax::Top
        }
        "B" => {
            Syntax::Bottom
        }
        "G" => {
            Syntax::Gamma
        }
        "D" => {
            Syntax::Delta
        }
        _ => {
            Syntax::Atom(token.clone())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
