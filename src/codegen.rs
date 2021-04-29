use crate::bytecode::Instruction;
use std::collections::HashMap;
use crate::builtins::get_builtin;

#[derive(Debug, Clone, Copy, Default)]
struct Alloc {
    start: usize,
    size: usize,
    //  [ - # # # # - -] -> start = 1, size = 4, next = 5
}

#[derive(Debug, Default)]
pub struct CodeGen<'a> {
    variables: HashMap<&'a str, Alloc>,
    functions: Vec<&'a str>,
    allocations: Vec<Alloc>,
    current_cell: usize,
}

impl<'a> CodeGen<'a> {
    fn clean(&mut self, size: usize) -> String {
        let mut res = String::new();
        for _ in 0..size {
            let alloc = self.allocations.pop().unwrap();
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
                self.allocate(1);
                res += &("+".repeat(x as usize) + ">");
            }

            Instruction::Getvar(x) => {
                let alloc = self.variables.get(x)
                    .unwrap().to_owned(); // testified in compiler
                let y = "<".repeat(alloc.size);
                let x = ">".repeat(alloc.size);

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
                let callee = get_builtin(self.functions.pop().unwrap(), size).unwrap();
                res += callee.as_str();
                dbg!(&self.allocations);
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
