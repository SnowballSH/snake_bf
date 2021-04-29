use crate::types::Type;

pub fn builtin_type(name: &str) -> Option<Type> {
    match name {
        "print" => Some(Type::BuiltinFunction(Box::new(Type::Unit))),
        _ => None
    }
}

pub fn get_builtin(name: &str, size: usize) -> Option<String> {
    match name {
        "print" => Some("<.".repeat(size) + &*">".repeat(size)),
        _ => None
    }
}