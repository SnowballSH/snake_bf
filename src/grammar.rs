#[macro_use]
use pest_derive::*;

use pest;
use pest::Parser;
use pest::iterators::{Pairs, Pair};

#[derive(Debug)]
pub enum Expression<'a> {
    Int(i64),
    Iden(&'a str),
    Call((Box<Expression<'a>>, Vec<Expression<'a>>)), // callee, args
}

#[derive(Debug)]
pub enum Statement<'a> {
    ExprStmt(Expression<'a>),
    LetAssign((&'a str, Expression<'a>)),
}

pub type Program<'a> = Vec<Statement<'a>>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TSParser;

fn parse_expression(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let base = inner.next().unwrap();
    let mut res = match base.as_rule() {
        Rule::integer => Expression::Int(base.as_str().parse().unwrap()),
        Rule::identifier => Expression::Iden(base.as_str()),
        _ => unreachable!()
    };

    loop {
        let dec = inner.next();
        match dec {
            Some(x) => {
                match x.as_rule() {
                    Rule::call => {
                        let args: Vec<Pair<Rule>> = x.into_inner().collect();
                        res = Expression::Call((
                            Box::new(res),
                            args.into_iter().map(|w| parse_expression(w)).collect(),
                        ))
                    }
                    _ => unreachable!()
                }
            }
            None => break
        }
    }

    res
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::expression_stmt => Statement::ExprStmt(
            parse_expression(pair.into_inner().next().unwrap())
        ),
        Rule::let_assign => {
            let mut inner = pair.into_inner();
            Statement::LetAssign((
                inner.next().unwrap().as_str(),
                parse_expression(inner.next().unwrap())
            ))
        }
        _ => unreachable!()
    }
}

fn parse_program(pairs: Pairs<Rule>) -> Program {
    let mut ast = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression_stmt |
            Rule::let_assign => {
                ast.push(parse_statement(pair));
            }
            _ => {}
        }
    }
    ast
}

pub fn parse(program: &str) -> Result<Program, pest::error::Error<Rule>> {
    let res = TSParser::parse(Rule::program, program);
    // dbg!(&res);
    match res {
        Ok(pairs) => {
            Ok(parse_program(pairs))
        }
        Err(x) => {
            Err(x)
        }
    }
}
