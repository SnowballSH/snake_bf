#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Byte,
    Unit,
    BuiltinFunction(Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Byte => "Byte",
            Type::Unit => "Unit",
            Type::BuiltinFunction(_) => "Builtin Function"
        }.to_string()
    }

    pub fn size(&self) -> usize {
        match self {
            Type::Byte => 1,
            Type::Unit => 0,
            Type::BuiltinFunction(_) => 0
        }
    }

    pub fn get_instance_type(&self, name: &str) -> Option<Type> {
        match self {
            Type::Byte => {
                match name {
                    "+" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
                    "-" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
                    "*" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
                    "/" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
                    "%" => Some(Type::BuiltinFunction(Box::new(Type::Byte))),
                    _ => None
                }
            }
            Type::Unit => {
                match name {
                    _ => None
                }
            }
            Type::BuiltinFunction(_) => {
                match name {
                    _ => None
                }
            }
        }
    }
}
