use crate::grammar::parse;
use crate::bytecode::ByteCodeGen;
use std::fs;
use crate::optimizer::optimize;

mod grammar;
mod codegen;
mod bytecode;
mod types;
mod builtins;
mod optimizer;

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
                    let mut res = genner.gen(x);
                    res = optimize(res);
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
