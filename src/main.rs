use crate::grammar::parse;
use crate::bytecode::ByteCodeGen;

mod grammar;
mod codegen;
mod bytecode;
mod types;

fn main() {
    let program = "
let x = 5
let y = x
";
    let res = parse(program);
    match res {
        Ok(x) => {
            let mut btc = ByteCodeGen::default();
            let res = btc.compile(x);
            match res {
                Ok(x) => {
                    let mut genner = codegen::CodeGen::default();
                    let res = genner.gen(x);
                    dbg!(res);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Err(x) => {
            println!("{}", x.to_string());
        }
    }
}
