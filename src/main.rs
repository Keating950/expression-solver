#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod types;
use crate::types::{Expr, Prop};
use std::{collections::HashSet, io, io::Read};
extern crate regex;
use regex::{Regex, RegexBuilder};
use rustyline::{error::ReadlineError, Editor, EditMode, Helper, Config};
extern crate nom;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple
};


const PROMPT: &'static str = ">>> ";

fn main() {
    let cfg = Config::builder()
        .auto_add_history(true)
        .tab_stop(4)
        .edit_mode(EditMode::Vi)
        .build();
    let mut rl = Editor::<()>::with_config(cfg);
    let vars = def_vars(&mut rl).unwrap();
    for v in vars { println!("{}", v) }
}

fn def_vars<H: Helper>(rl: &mut Editor<H>) -> io::Result<Vec<Prop>> {
    fn input_to_bool(s: &str) -> Option<bool> {
        let s_lc = s.to_lowercase();
        if ["t", "true"].contains(&&*s_lc) { return Some(true); }
        if ["f", "false"].contains(&&*s_lc) { return Some(false); }
        None
    }

    let mut vars: Vec<Prop> = Vec::with_capacity(2);
    const HELP_MSG: &'static str = "Define your variables (syntax: p = true):";
    println!("{}", HELP_MSG);
    while let Ok(line) = rl.readline(PROMPT) {
        if line.is_empty() { break; }
        let (var, val) = (|| {
            let arr: Vec<&str> = line.split("=").take(2).map(|s| s.trim()).collect();
            (arr[0], arr[1])
        })();
        match input_to_bool(val) {
            Some(val) => vars.push(Prop::new(var, val)),
            None => eprintln!("{}", HELP_MSG),
        }
    }
    Ok(vars)
}

fn parse_line(line: &str) -> Box<Expr> {
    unimplemented!();
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
}
