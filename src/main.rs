#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate regex;
use regex::{Regex, RegexBuilder};
use rustyline::{error::ReadlineError, Editor, EditMode, Helper, Config};
#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::{collections::HashSet, io, io::Read};
mod types;
mod parsing;
use crate::{types::*, parsing::*};
use pest::{
    prec_climber::{PrecClimber, Operator, Assoc},
    iterators::Pair,
    Parser,
};

const PROMPT: &'static str = ">>> ";

macro_rules! mkerr {
    ($msg:tt) => { io::Error::new(io::ErrorKind::InvalidInput, $msg) };
    ($kind:tt, $msg:tt) => { io::Error::new(io::ErrorKind::$kind, $msg) };
}

fn main() {
    /*    let cfg = Config::builder()
            .auto_add_history(true)
            .tab_stop(4)
            .edit_mode(EditMode::Vi)
            .build();
        let rl = Editor::<()>::with_config(cfg);
    */
    let _climber = PrecClimber::new(vec![
        Operator::new(Rule::and, Assoc::Left)
            | Operator::new(Rule::or, Assoc::Left)
            | Operator::new(Rule::superset, Assoc::Left),
        Operator::new(Rule::neg, Assoc::Right),
    ]);
    let pairs = ExpressionParser::parse(Rule::expression, "(!P|P)&Q").expect("Parse failure");
    for pair in pairs { print_pair(pair) }
}

fn print_pair(p: Pair<Rule>) {
    fn inner(p: Pair<Rule>) {
        match p.as_rule() {
            Rule::expression => {
                println!("Expression:\t{}", p.as_str());
                for subp in p.into_inner() { inner(subp) }
            }
            Rule::primary => println!("Primary:\t{}", p.as_str()),
            Rule::ident => println!("Ident:\t{}", p.as_str()),
            Rule::and => println!("And:\t{}", p.as_str()),
            Rule::or => println!("Or:\t{}", p.as_str()),
            Rule::superset => println!("Superset:\t{}", p.as_str()),
            Rule::neg => println!("Neg:\t{}", p.as_str()),
            // _ => unreachable!()
        };
    }
    println!("Rule:\t{:?}", p.as_rule());
    println!("Span:\t{:?}", p.as_span());
    println!("Text:\t{}", p.as_str());
    for inner_pair in p.into_inner() {inner(inner_pair)}
}

fn def_vars<H: Helper>(rl: &mut Editor<H>) -> Option<Vec<Prop>> {
    // let mut vars: Vec<Prop> = Vec::with_capacity(2);
    const HELP_MSG: &'static str = "Define your variables (syntax: p = true):";
    println!("{}", HELP_MSG);
    unimplemented!();
    /*    while let Ok(line) = rl.readline(PROMPT) {
            if line.is_empty() { break; }
            match parse_prop(line.as_bytes()) {
                Ok(([], (name, val))) => vars.push(Prop::new(name, val)),
                _ => eprintln!("{}", HELP_MSG),
            }
        }
        if vars.len() > 0 { Some(vars) } else { None }
    */
}

fn eval_expr(e: &Expr) -> bool {
    match e {
        Expr::And(e1, e2) => eval_expr(e1) && eval_expr(e2),
        Expr::Or(e1, e2) => eval_expr(e1) || eval_expr(e2),
        Expr::Super(e1, e2) => match (eval_expr(e1), eval_expr(e2)) {
            (true, false) => false,
            (_, _) => true,
        },
        Expr::Not(e1) => !eval_expr(e1),
        Expr::Prop(e1) => e1.val,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::iter::FromIterator;

    #[test]
    fn test_eval_expr() {
        let e = Expr::And(
            Expr::from_prop(Prop::new("P", true)),
            Expr::from_prop(Prop::new("Q", false)),
        );
        assert_eq!(eval_expr(&e), false);
        let e1 = Expr::Super(
            Expr::from_prop(Prop::new("P", false)),
            Expr::from_prop(Prop::new("Q", false)),
        );
        assert_eq!(eval_expr(&e1), true);
        let e2 = Expr::And(
            Box::new(Expr::Super(
                Expr::from_prop(Prop::new("P", false)),
                Expr::from_prop(Prop::new("Q", false)),
            )),
            Box::new(Expr::Or(
                Expr::from_prop(Prop::new("P", false)),
                Expr::from_prop(Prop::new("Q", true)),
            )),
        );
        assert_eq!(eval_expr(&e2), true);
    }
    // #[test]
    // fn test_parse_expr() {
    //     let e1 = "p and q";
    //     let e1_set: HashSet<String> = HashSet::from_iter(["p", "q"].iter().map(|s| s.to_string()));
    //     match parse_expr(e1.as_bytes(), e1_set) { _ => () };
    // }
}
