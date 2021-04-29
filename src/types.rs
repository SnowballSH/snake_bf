#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Byte,
    Unit,
    BuiltinFunction(Box<Type>)
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Byte => "Byte",
            Type::Unit => "Unit",
            Type::BuiltinFunction(_) => "Builtin Function"
        }.to_string()
    }
}
