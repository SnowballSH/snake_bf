use crate::types::Type;

pub fn builtin_type(name: &str) -> Option<Type> {
    match name {
        "print" => Some(Type::BuiltinFunction(Box::new(Type::Unit))),
        "sum" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
        "Byte+" => Type::Byte.get_instance_type("+"),
        "Byte-" => Type::Byte.get_instance_type("-"),
        _ => None
    }
}

pub fn get_builtin(name: &str, size: usize) -> Option<String> {
    match name {
        "print" => {
            let mut pre = ">++++[<++++++++>-]<".to_string();  // make a space
            for i in 0..size {
                pre += &("<".repeat(i + 1) + "." + &*">".repeat(i + 1));
                if i != size - 1 {
                    pre += "."
                } else {
                    pre += ">+++[<------->-]<-."  // change space to newline
                }
            }
            Some(pre + "[-]")
        }
        "sum" => {
            let mut res = "".to_string();
            for i in 0..size {
                res += &("<".repeat(i + 1)
                    + &*format!("[-{}+{}]", ">".repeat(i + 1), "<".repeat(i + 1))
                    + &*">".repeat(i + 1));
            }
            Some(res + ">")
        }
        "Byte+" => {
            Some("<[->+<]<[->>+<<]>>>".to_string())
        }
        "Byte-" => {
            Some("<<[->>+<<]>[->-<]>>".to_string())
        }
        _ => None
    }
}