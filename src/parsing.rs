#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::types::*;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    {state, ParseResult, Parser, ParserState},
};
use std::collections::HashMap;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    expression,
    primary,
    ident,
    and,
    or,
    superset,
    neg,
}

pub struct ExpressionParser;

impl Parser<Rule> for ExpressionParser {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        fn expression(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::expression, |s| {
                s.sequence(|s| {
                    primary(s).and_then(|s| {
                        s.repeat(|s| {
                            s.sequence(|s| {
                                and(s)
                                    .or_else(|s| or(s))
                                    .or_else(|s| superset(s))
                                    .or_else(|s| neg(s))
                                    .and_then(|s| primary(s))
                            })
                        })
                    })
                })
            })
        }

        fn primary(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.sequence(|s| {
                s.match_char_by(|c| c == '(')
                 .and_then(|s| expression(s))
                 .and_then(|s| s.match_char_by(|c| c == ')'))
            }).or_else(|s| ident(s))
        }

        fn ident(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::ident, |s| {
                s.sequence(|s| {
                    // s.optional(|s| s.match_string("!")).and_then(|s| {
                    s.repeat(|s| s.match_range('a'..'z')
                                  .or_else(|s| s.match_range('A'..'Z')))
                })
            })
            // })
        }

        fn and(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::and, |s| s.match_string("&"))
        }
        fn or(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::or, |s| s.match_string("|"))
        }
        fn superset(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::superset, |s| s.match_string(">"))
        }

        fn neg(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.rule(Rule::neg, |s| s.match_string("!"))
        }
        state(input, |state| match rule {
            Rule::expression => expression(state),
            _ => unreachable!(),
        })
    }
}

fn consume(pair: Pair<Rule>, climber: &PrecClimber<Rule>, ident_values: &HashMap<&str, bool>) -> bool {
    let primary = |pair| consume(pair, climber, &ident_values);
    let infix = |lhs: bool, op: Pair<Rule>, rhs: bool| match op.as_rule() {
        Rule::and => lhs && rhs,
        Rule::or => lhs | rhs,
        Rule::superset => (lhs, rhs) != (true, false),
        _ => unreachable!(),
    };
    match pair.as_rule() {
        Rule::expression => climber.climb(pair.into_inner(), primary, infix),
        Rule::ident => pair.as_str().parse().unwrap(),
        _ => unreachable!(),
    }
}
