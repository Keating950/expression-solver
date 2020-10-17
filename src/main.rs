#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod types;

use crate::types::{Expr, Prop};
use std::{io, io::Read, collections::HashSet};

extern crate regex;

use regex::{Regex, RegexBuilder};


fn main() {
    // match def_vars() {
    //     Ok(toks) => for t in toks { println!("{}", t) }
    // }
    let e = Expr::And(
        Expr::from_prop(Prop::new("P", true)),
        Expr::from_prop(Prop::new("Q", false)),
    );
    println!("{} = {}", e, eval_expr(&e));
    let e1 = Expr::Super(
        Expr::from_prop(Prop::new("P", false)),
        Expr::from_prop(Prop::new("Q", false)),
    );
    println!("{} = {}", e1, eval_expr(&e1));
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
    println!("{} = {}", e2, eval_expr(&e2));
}

#[allow(unreachable_code)]
fn repl() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    loop {
        print!(">>> ");
        stdin.read_to_string(&mut buffer)?;
    }
    Ok(())
}

fn def_vars() -> io::Result<Vec<Prop>> {
    let mut vec: Vec<Prop> = Vec::with_capacity(4);
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    println!("Define your variables (e.g. P = true)");
    loop {
        print!(">>> ");
        stdin.read_to_string(&mut buffer)?;
        buffer = buffer.to_ascii_lowercase();
        if buffer.is_empty() {
            break;
        }
        let toks: Vec<String> = buffer.split("=").map(|t| t.trim().to_string()).collect();
        if toks.len() != 2 {
            println!("Define your variables (e.g. P = true)");
            continue;
        }
        match input_to_bool(&toks[1]) {
            Some(v) => vec.push(Prop::new(&*toks[0], v)),
            None => println!("Define your variables (e.g. P = true)")
        }
    }
    Ok(vec)
}

fn input_to_bool(s: &str) -> Option<bool> {
    if s == "t" || s == "true" { return Some(true); }
    if s == "f" || s == "false" { return Some(false); }
    None
}

fn parse_line(line: &str) -> Box<Expr> {
    unimplemented!();
}

fn eval_expr(e: &Expr) -> bool {
    match e {
        Expr::And(e1, e2) => eval_expr(e1) && eval_expr(e2),
        Expr::Or(e1, e2) => eval_expr(e1) || eval_expr(e2),
        Expr::Super(e1, e2) =>
            match (eval_expr(e1), eval_expr(e2)) {
                (true, false) => false,
                (_, _) => true
            }
        Expr::Not(e1) => !eval_expr(e1),
        Expr::Prop(e1) => e1.val
    }
}