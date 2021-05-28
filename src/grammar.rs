use pest_derive::*;

use pest;
use pest::Parser;
use pest::iterators::{Pairs, Pair};

#[derive(Debug)]
pub enum Expression<'a> {
    Int(i64),
    Iden(&'a str),
    Call((Box<Expression<'a>>, Vec<Expression<'a>>)), // callee, args
    Infix(Box<Infix<'a>>)
}

#[derive(Debug)]
pub struct Infix<'a> {
    pub left: Expression<'a>,
    pub operator: &'a str,
    pub right: Expression<'a>,
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

fn infix<'a>(lhs: Expression<'a>, op: Pair<'a, Rule>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::Infix(Box::new(Infix {
        left: lhs,
        operator: op.as_str(),
        right: rhs,
    }))
}

fn others(pair: Pair<Rule>) -> Expression {
    let res = match pair.as_rule() {
        Rule::integer => Expression::Int(pair.as_str().parse().unwrap()),
        Rule::identifier => Expression::Iden(pair.as_str()),
        Rule::suffix => {
            let mut inner = pair.clone().into_inner();
            let res = inner.next().unwrap();
            let _args: Vec<Pair<Rule>> = inner.collect();
            let mut args_iter = _args.into_iter();

            let n = args_iter.next().unwrap();
            let mut callee = match n.as_rule() {
                Rule::call => Expression::Call(
                    (Box::new(parse_expression(res)), n.into_inner()
                    .map(|w| parse_expression(w))
                    .collect())),
                _ => unreachable!()
            };

            while let Some(xx) = args_iter.next() {
                callee = match xx.as_rule() {
                    Rule::call => Expression::Call(
                        (Box::new(callee), xx.into_inner()
                            .map(|w| parse_expression(w))
                            .collect())),
                    _ => unreachable!()
                }
            }

            callee
        }
        _ => unreachable!()
    };

    res
}

fn parse_expression(pair: Pair<Rule>) -> Expression {
    let inner: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
    let res = if inner.len() != 0 && pair.clone().as_rule() == Rule::expression {
        climb(pair)
    } else {
        others(pair)
    };

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

use lazy_static::*;
use pest::prec_climber::*;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(Rule::add, Left) | Operator::new(Rule::sub, Left),
            Operator::new(Rule::mul, Left) | Operator::new(Rule::div, Left) | Operator::new(Rule::modulo, Left),
        ])
    };
}

pub fn climb(pair: Pair<Rule>) -> Expression {
    PREC_CLIMBER.climb(pair.into_inner(), others, infix)
}
