use std::collections::HashMap;

use crate::builtins::builtin_type;
use crate::grammar::*;
use crate::types::Type;
use crate::bytecode::Instruction::{PromoteFunction, Call};

#[derive(Clone, Debug)]
pub enum Instruction<'a> {
    Byte(u8),
    Getvar(&'a str),
    PromoteFunction(String),
    SetVar(&'a str),
    Call(usize),
    Pop,
}

#[derive(Default, Debug, Clone)]
pub struct ByteCodeGen {
    variables: HashMap<String, Type>,
}

pub type VecIns<'a> = Result<Vec<Instruction<'a>>, String>;
pub type VecInsExpr<'a> = Result<(Vec<Instruction<'a>>, Type), String>;

impl ByteCodeGen {
    pub fn compile<'a>(&mut self, program: Program<'a>) -> VecIns<'a> {
        let mut res = vec![];
        for node in program {
            let n = self.compile_stmt(node);
            match n {
                Ok(x) => res.extend(x),
                Err(x) => return Err(x),
            }
        }
        Ok(res)
    }

    fn compile_stmt<'a>(&mut self, stmt: Statement<'a>) -> VecIns<'a> {
        let mut res = vec![];
        match stmt {
            Statement::ExprStmt(node) => {
                match node {
                    Expression::Call(_) => {}
                    _ => {
                        return Ok(res);
                    }
                };

                let n = self.compile_expr(node);
                match n {
                    Ok(x) => {
                        res.extend(x.0);
                        if x.1 != Type::Unit {
                            res.push(Instruction::Pop);
                        }
                    }
                    Err(x) => return Err(x),
                }
            }
            Statement::LetAssign(node) => {
                let n = self.compile_expr(node.1);
                match n {
                    Ok(x) => {
                        if x.1 == Type::Unit {
                            return Err("Cannot assign to Unit type".to_string());
                        }
                        self.variables.insert(node.0.to_string(), x.1);
                        res.extend(x.0);
                        res.push(Instruction::SetVar(node.0));
                    }
                    Err(x) => return Err(x),
                }
            }
        }
        Ok(res)
    }

    fn compile_expr<'a>(&self, expr: Expression<'a>) -> VecInsExpr<'a> {
        let mut res = vec![];
        let tp: Type = match expr {
            Expression::Int(x) => {
                if x > 255 || x < 0 {
                    return Err(format!("Number '{}' is not in byte range", x));
                }
                res.push(Instruction::Byte(x as u8));
                Type::Byte
            }
            Expression::Iden(x) => {
                if let Some(t) = builtin_type(x) {
                    res.push(Instruction::PromoteFunction(x.to_string()));
                    t
                } else {
                    if !self.variables.contains_key(x) {
                        return Err(format!("Variable '{}' is not defined", x));
                    } else {
                        res.push(Instruction::Getvar(x));
                        self.variables.get(x).unwrap().to_owned()
                    }
                }
            }
            Expression::Call(mut x) => {
                let callee = self.compile_expr(*x.0);
                match callee {
                    Ok(k) => {
                        if let Type::BuiltinFunction(t) = k.1 {
                            let size = x.1.len();
                            x.1.reverse();
                            for ex in x.1 {
                                let r = self.compile_expr(ex);
                                match r {
                                    Ok(x) => {
                                        if x.1 == Type::Unit {
                                            return Err("Cannot use Unit type as argument".to_string());
                                        }
                                        res.extend(x.0);
                                    }
                                    Err(x) => return Err(x),
                                };
                            }
                            res.extend(k.0);
                            res.push(Instruction::Call(size));
                            *t
                        } else {
                            return Err(format!("Type '{}' is not callable", k.1.to_string()));
                        }
                    }
                    Err(x) => return Err(x),
                }
            }
            Expression::Infix(x) => {
                let left = self.compile_expr(x.left);
                match left {
                    Ok(left) => {
                        res.extend(left.0);
                        let right = self.compile_expr(x.right);
                        match right {
                            Ok(right) => {
                                res.extend(right.0);
                                let tt = right.1.get_instance_type(x.operator);
                                match tt {
                                    Some(t) => {
                                        if let Type::BuiltinFunction(ttt) = t.clone() {
                                            res.push(PromoteFunction(right.1.to_string() + x.operator));
                                            res.push(Call(2));
                                            *ttt
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                    None => return Err(format!(
                                        "Instance '{}' does not exist on type {}",
                                        x.operator, left.1.to_string()))
                                }
                            }
                            Err(e) => return Err(e)
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
        };
        Ok((res, tp))
    }
}
