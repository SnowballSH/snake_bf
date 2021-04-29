use crate::grammar::parse;
use crate::bytecode::ByteCodeGen;

mod grammar;
mod codegen;
mod bytecode;
mod types;
mod builtins;

fn main() {
    let program = "
let x = 65
let y = x
print(x, y)
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
