#[macro_use]
use pest_derive::*;

use pest;
use pest::Parser;
use pest::iterators::{Pairs, Pair};

#[derive(Debug)]
pub enum Expression<'a> {
    Int(i64),
    Iden(&'a str),
    Call, // TODO
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
    match pair.as_rule() {
        Rule::integer => Expression::Int(pair.as_str().parse().unwrap()),
        Rule::identifier => Expression::Iden(pair.as_str()),
        _ => unreachable!()
    }
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
    match res {
        Ok(pairs) => {
            Ok(parse_program(pairs))
        }
        Err(x) => {
            Err(x)
        }
    }
}
