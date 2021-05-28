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
let thirteen = 13;
let A = thirteen + thirteen - 6;
let B = A + A;
print(sum(A, B, 2), sum(A, A, A));
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
