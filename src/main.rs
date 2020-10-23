use rustyline::{Config, EditMode, Editor, Helper};
use std::{
    collections::{HashMap, VecDeque},
    env::args,
    error::Error,
    io,
    process::exit,
};

macro_rules! readline {
    ($rl:ident) => { $rl.readline(">>> ") };
}
macro_rules! mkerr {
    ($msg:expr) => {
        io::Error::new(io::ErrorKind::InvalidInput, $msg)
    };
}

const HELP_MSG: &'static str = "SYNTAX:
-------
negation:\t!
and:\t\t&
or:\t\t|
superset:\t>";

fn main() {
    if args().len() != 1 {
        eprintln!("{}", HELP_MSG);
        exit(1);
    }
    let mut rl = Editor::<()>::with_config(
        Config::builder()
            .auto_add_history(true)
            .tab_stop(4)
            .edit_mode(EditMode::Vi)
            .build(),
    );
    let vars = def_vars(&mut rl).unwrap();
    loop {
        println!("Enter your expression:");
        let line = match readline!(rl) {
            Ok(l) => l,
            Err(_) => {
                println!("Invalid expression");
                continue;
            }
        };
        let mut expr = match shunting_yard(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        match eval_postfix(&mut expr, &vars) {
            Ok(v) => println!("Result: {}", v),
            Err(_) => println!("Failed to evaluate expression"),
        }
    }
}

fn def_vars<H: Helper>(rl: &mut Editor<H>) -> io::Result<HashMap<char, bool>> {
    macro_rules! print_help {
        () => {
            println!("Define your variables (syntax: p = true):")
        };
        ($msg:literal) => {
            println!(
                concat!($msg, "\n{}"),
                "Define your variables (syntax: p = true):"
            )
        };
    }
    let mut vars: HashMap<char, bool> = HashMap::new();
    print_help!();
    while let Ok(mut line) = readline!(rl) {
        line.retain(|c| !c.is_whitespace());
        if line.is_empty() {
            break;
        }
        let tokens: Vec<&str> = line.split("=").collect();
        let (var, val) = if tokens.len() == 2 {
            (tokens[0], tokens[1])
        } else {
            print_help!("Invalid syntax");
            continue;
        };
        let key = match var.len() {
            0 => {
                print_help!("No variable name received");
                continue;
            }
            1 => var.chars().nth(0).unwrap(),
            _ => {
                print_help!("Variable names must a single letter");
                continue;
            }
        };
        match val.parse() {
            Ok(v) => { vars.insert(key, v); }
            _ => print_help!(),
        }
    }
    if vars.len() > 0 { Ok(vars) } else { Err(mkerr!("No variables received")) }
}

fn shunting_yard(line: &str) -> io::Result<VecDeque<char>> {
    let mut operator_stack: VecDeque<char> = VecDeque::new();
    let mut output_queue: VecDeque<char> = VecDeque::with_capacity(line.len());
    for c in line.chars().filter(|c| !c.is_whitespace()) {
        match c {
            '&' | '|' | '>' => {
                while match operator_stack.front() { Some('(') | None => false, _ => true } {
                    output_queue.push_back(operator_stack.pop_back().unwrap())
                }
                operator_stack.push_front(c)
            }
            '!' | '(' => operator_stack.push_front(c),
            ')' => match operator_stack.front() {
                Some('(') => (),
                Some(_) => operator_stack.push_front(c),
                None => return Err(mkerr!("Unterminated parenthesis")),
            },
            'a'..='z' | 'A'..='Z' => output_queue.push_back(c),
            other => {
                return Err(mkerr!(format!("Encountered unexpected symbol {}", other)));
            }
        }
    }
    while let Some(c) = operator_stack.pop_front() {
        output_queue.push_back(c)
    }
    output_queue.retain(|c| !['(', ')'].contains(c));
    Ok(output_queue)
}

fn eval_postfix(expr: &mut VecDeque<char>, vars: &HashMap<char, bool>) -> Result<bool, Box<dyn Error>> {
    macro_rules! pop {
        ($s:ident, $char:ident) => {
            $s.pop().ok_or(format!(
                "Syntax error: {} requires more arguments than provided",
                $char
            ))?
        };
    }
    let mut stack: Vec<bool> = Vec::with_capacity(vars.len());
    while let Some(c) = expr.pop_front() {
        match c {
            '!' => {
                let a = pop!(stack, c);
                stack.push(!a)
            }
            '|' => {
                let (b, a) = (pop!(stack, c), pop!(stack, c));
                stack.push(a || b)
            }
            '&' => {
                let (b, a) = (pop!(stack, c), pop!(stack, c));
                stack.push(a && b)
            }
            '>' => {
                let (b, a) = (pop!(stack, c), pop!(stack, c));
                let result = match (a, b) {
                    (true, false) => false,
                    (_, _) => true
                };
                stack.push(result);
            }
            _ => stack.push(*vars.get(&c).ok_or(format!("Variable {} not recognized", c))?),
        }
    }
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err(Box::new(mkerr!( "Could not evaluate expression: Excess tokens received")))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_shunting_yard() {
        assert_eq!(shunting_yard("Q & P").unwrap(), vec!['Q', 'P', '&']);
        assert_eq!(shunting_yard("!Q & P").unwrap(), vec!['Q', '!', 'P', '&']);
        assert_eq!(shunting_yard("Q | P").unwrap(), vec!['Q', 'P', '|']);
        assert_eq!(shunting_yard("!Q & !P").unwrap(), vec!['Q', '!', 'P', '!', '&']);
    }
    #[test]
    fn test_eval() {
        let mut map1 = HashMap::new();
        map1.insert('P', true);
        map1.insert('Q', true);
        let mut stack1 = shunting_yard("Q & P").unwrap();
        assert_eq!(eval_postfix(&mut stack1, &map1).unwrap(), true);
        let mut stack2 = shunting_yard("!Q & !P").unwrap();
        assert_eq!(eval_postfix(&mut stack2, &map1).unwrap(), false);
        let mut stack3 = shunting_yard("Q > P").unwrap();
        assert_eq!(eval_postfix(&mut stack3, &map1).unwrap(), true);
        let mut stack4 = shunting_yard("Q > !P").unwrap();
        assert_eq!(eval_postfix(&mut stack4, &map1).unwrap(), false);
    }
}
