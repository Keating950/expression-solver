#![feature(try_trait)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use rustyline::{error::ReadlineError, Editor, EditMode, Helper, Config};
use std::{collections::{HashSet, HashMap}, io, io::Read, option::NoneError};
mod types;
use crate::types::*;


const PROMPT: &'static str = ">>> ";

macro_rules! mkerr {
    ($msg:tt) => { io::Error::new(io::ErrorKind::InvalidInput, $msg) };
    ($kind:tt, $msg:tt) => { io::Error::new(io::ErrorKind::$kind, $msg) };
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
}


fn def_vars<H: Helper>(rl: &mut Editor<H>) -> io::Result<HashMap<char, bool>> {
    // let mut vars: Vec<Prop> = Vec::with_capacity(2);
    const HELP_MSG: &'static str = "Define your variables (syntax: p = true):";
    println!("{}", HELP_MSG);
    let mut vars: HashMap<char, bool> = HashMap::new();
    while let Ok(mut line) = rl.readline(PROMPT) {
        line.retain(|c| !c.is_whitespace());
        if line.is_empty() { break; }
        let tokens: Vec<&str> = line.split("=").collect();
        let (var, val) = (tokens[0], tokens[1].parse());
        let key = match var.len() {
            0 => { eprintln!("No variable name received"); continue }
            1 => var.chars().nth(0).unwrap(),
            _ => { eprintln!("Variable names must a single letter."); continue}
        };
        match val {
            Ok(v) => { vars.insert(key,  v); }
            _ => eprintln!("{}", HELP_MSG),
        }
    }
    if vars.len() > 0 { Ok(vars) } else { Err(mkerr!("No variables received")) }
}

fn shunting_yard(line: &str) -> io::Result<Vec<char>> {
    let mut operator_stack: Vec<char> = Vec::new();
    let mut output_queue: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars().filter(|c| !c.is_whitespace()) {
        match c {
            '&' | '|' => {
                while !(operator_stack.is_empty() || *operator_stack.first().unwrap() == '(') {
                    output_queue.push(operator_stack.pop().unwrap())
                }
                operator_stack.push(c)
            }
            '!' | '(' => operator_stack.push(c),
            ')' => match operator_stack.first() {
                Some('(') => (),
                Some(_) => operator_stack.push(c),
                None => return Err(mkerr!("Unterminated parenthesis"))
            },
            'a'..='z' | 'A'..='Z' => output_queue.push(c),
            _ => return Err(mkerr!("Encountered unexpected symbol"))
        }
    }
    output_queue.append(&mut operator_stack);
    output_queue.retain(|c| *c != '(' && *c != ')');
    Ok(output_queue)
}

fn eval_postfix(expr: &mut Vec<char>, vars: &HashMap<char, bool>) -> Result<bool, NoneError> {
    let mut stack: Vec<bool> = Vec::with_capacity(expr.len());
    while let Some(c) = expr.pop() {
        match c {
            '!' => {
                let a = stack.pop()?;
                stack.push(!a)
            }
            '|' => {
                let (a, b) = (stack.pop()?, stack.pop()?);
                stack.push(a || b)
            }
            '&' => {
                let (a, b) = (stack.pop()?, stack.pop()?);
                stack.push(a && b)
            }
            '>' => {
                let result = match (stack.pop()?, stack.pop()?) {
                    (true, false) => false,
                    (_, _) => true,
                };
                stack.push(result);
            }
            _ => stack.push(*vars.get(&c)?)
        }
    }
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err(NoneError)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::iter::FromIterator;
    #[test]
    fn test_shunting_yard() {
        let v = shunting_yard("Q & P");
        assert_eq!(v.unwrap(), vec!['Q', 'P', '&']);
        let v1 = shunting_yard("!Q & P");
        for t in v1.unwrap() {
            print!("{} ", t);
        }
        println!();
    }

    #[test]
    fn test_eval_expr() {
    }
    // #[test]
    // fn test_parse_expr() {
    //     let e1 = "p and q";
    //     let e1_set: HashSet<String> = HashSet::from_iter(["p", "q"].iter().map(|s| s.to_string()));
    //     match parse_expr(e1.as_bytes(), e1_set) { _ => () };
    // }
}
