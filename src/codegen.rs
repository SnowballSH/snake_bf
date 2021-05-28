use std::collections::HashMap;

use crate::builtins::{builtin_type, get_builtin};
use crate::bytecode::Instruction;
use crate::types::Type;

#[derive(Debug, Clone, Copy, Default)]
struct Alloc {
    start: usize,
    size: usize,
    //  [ - # # # # - -] -> start = 1, size = 4, next = 5
}

#[derive(Debug, Default)]
pub struct CodeGen<'a> {
    variables: HashMap<&'a str, Alloc>,
    functions: Vec<String>,
    allocations: Vec<Alloc>,
    current_cell: usize,
}

impl<'a> CodeGen<'a> {
    fn clean(&mut self, size: usize) -> String {
        let mut res = String::new();
        for _ in 0..size {
            let alloc = self.allocations.pop().unwrap();
            self.current_cell -= alloc.size;
            res += "<[-]".repeat(alloc.size).as_str();
        }
        res
    }

    pub fn gen(&mut self, input: Vec<Instruction<'a>>) -> String {
        let mut res = String::new();
        for ins in input {
            res += &*self.gen_ins(ins);
        }
        res
    }

    pub fn gen_ins(&mut self, ins: Instruction<'a>) -> String {
        let mut res = String::new();
        match ins {
            Instruction::Byte(x) => {
                self.allocate(Type::Byte.size());
                res += &("+".repeat(x as usize) + ">");
            }

            Instruction::Getvar(x) => {
                let alloc = self.variables.get(x)
                    .unwrap().to_owned(); // testified in compiler
                // dbg!(x, self.current_cell, alloc.start);
                let y = "<".repeat(self.current_cell - alloc.start);  // [- # # - - *]  * = 5, # = 1, 2
                let x = ">".repeat(self.current_cell - alloc.start);

                // [ - * - - # ]  # = * -> <<<
                res += format!("{}[{}+>+<{}-]{}>[<{}+{}>-]", y, x, y, x, y, x).as_str();
                self.allocate(1);
            }

            Instruction::PromoteFunction(x) => {
                self.functions.push(x)
            }

            Instruction::SetVar(x) => {
                self.variables.insert(x,
                                      self.allocations.last()
                                          .expect("Empty stack").to_owned());
            }

            Instruction::Pop => {
                res += &*self.clean(1);
            }

            Instruction::Call(size) => {
                let name = self.functions.pop().unwrap();
                let callee = get_builtin(&*name, size).unwrap();
                res += callee.as_str();
                //dbg!(&self);

                let t = builtin_type(&*name).unwrap();
                let s = if let Type::BuiltinFunction(tt) = t {
                    tt.size()
                } else {
                    t.size()
                };
                self.allocate(s);

                // [ - a b c # ] move # to a
                let x = "<".repeat(size);
                let y = ">".repeat(size);
                res += &*format!("{}{}[-]{}[{}+{}-]", "<".repeat(s), x, y, x, y);
                res += &*">".repeat(s);
                res += &*self.clean(size);
            }
        };
        res
    }

    fn allocate(&mut self, size: usize) -> Alloc {
        let alloc = Alloc {
            start: self.current_cell,
            size,
        };
        self.current_cell += size;
        self.allocations.push(alloc);
        alloc
    }
}
