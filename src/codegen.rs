use crate::bytecode::Instruction;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
struct Alloc {
    start: usize,
    size: usize,
    //  [ - # # # # - -] -> start = 1, size = 4, next = 5
}

#[derive(Debug, Default)]
pub struct CodeGen<'a> {
    variables: HashMap<&'a str, Alloc>,
    allocations: Vec<Alloc>,
    current_cell: usize,
}

impl<'a> CodeGen<'a> {
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
            }
            Instruction::SetVar(x) => {
                self.variables.insert(x,
                                      self.allocations.last()
                                          .expect("Empty stack").to_owned());
            }
            Instruction::Pop => {
                let alloc = self.allocations.pop().unwrap();
                res += "<[-]".repeat(alloc.size).as_str();
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
