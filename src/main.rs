use crate::grammar::parse;
use crate::bytecode::ByteCodeGen;
use std::fs;

mod grammar;
mod codegen;
mod bytecode;
mod types;
mod builtins;

fn main() {
    let program = "
print(sum(45, 20))
";
    let res = parse(program);
    // dbg!(&res);
    match res {
        Ok(x) => {
            let mut btc = ByteCodeGen::default();
            let res = btc.compile(x);
            match res {
                Ok(x) => {
                    let mut genner = codegen::CodeGen::default();
                    let res = genner.gen(x);
                    dbg!(&res);
                    fs::write("test.bf", res).expect("Unable to write file");
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
