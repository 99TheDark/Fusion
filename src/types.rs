#[derive(PartialEq, Clone)]
pub enum DataType {
    Int(Int),
    Uint(Uint),
    Float(Float),
    Bool(Bool),
    Array(Array),
}

impl DataType {
    pub fn from(src: &String) -> Option<DataType> {
        // There HAS to be a better way of doing this
        if let Some(x) = Int::from(&src) {
            Some(DataType::Int(x))
        } else if let Some(x) = Uint::from(&src) {
            Some(DataType::Uint(x))
        } else if let Some(x) = Float::from(&src) {
            Some(DataType::Float(x))
        } else if let Some(x) = Bool::from(&src) {
            Some(DataType::Bool(x))
        } else if let Some(x) = Array::from(&src) {
            Some(DataType::Array(x))
        } else {
            None
        }
    }

    pub fn eq(&self, x: &DataType) -> bool {
        match (self, x) {
            (DataType::Int(x), DataType::Int(y)) => x.eq(y),
            (DataType::Uint(x), DataType::Uint(y)) => x.eq(y),
            (DataType::Float(x), DataType::Float(y)) => x.eq(y),
            (DataType::Bool(_), DataType::Bool(_)) => true,
            (DataType::Array(x), DataType::Array(y)) => x.eq(y),
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataType::Int(x) => x.to_string(),
            DataType::Uint(x) => x.to_string(),
            DataType::Float(x) => x.to_string(),
            DataType::Bool(x) => x.to_string(),
            DataType::Array(x) => x.to_string(),
        }
    }
}

impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I hate this implementation...
        match self {
            DataType::Int(x) => write!(f, "{:#?}", x),
            DataType::Uint(x) => write!(f, "{:#?}", x),
            DataType::Float(x) => write!(f, "{:#?}", x),
            DataType::Bool(x) => write!(f, "{:#?}", x),
            DataType::Array(x) => write!(f, "{:#?}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IntegralSize {
    Int8 = 8,
    Int16 = 16,
    Int32 = 32,
    Int64 = 64,
    Int128 = 128,
}

impl IntegralSize {
    pub const VALUES: [IntegralSize; 5] = [
        IntegralSize::Int8,
        IntegralSize::Int16,
        IntegralSize::Int32,
        IntegralSize::Int64,
        IntegralSize::Int128,
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Int {
    pub size: IntegralSize,
}

impl Int {
    pub fn new(size: IntegralSize) -> DataType {
        DataType::Int(Int { size })
    }

    pub fn from(src: &String) -> Option<Int> {
        if &src[0..3] == "int" {
            for size in IntegralSize::VALUES {
                if (size as u32).to_string() == &src[3..] {
                    return Some(Int { size });
                }
            }
        }

        None
    }

    pub fn eq(&self, x: &Int) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("int{}", self.size as u32)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Uint {
    pub size: IntegralSize,
}

impl Uint {
    pub fn new(size: IntegralSize) -> DataType {
        DataType::Uint(Uint { size })
    }

    pub fn from(src: &String) -> Option<Uint> {
        if &src[0..3] == "int" {
            for size in IntegralSize::VALUES {
                if (size as u32).to_string() == &src[3..] {
                    return Some(Uint { size });
                }
            }
        }

        None
    }

    pub fn eq(&self, x: &Uint) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("uint{}", self.size as u32)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FloatingSize {
    Float16 = 16,
    Float32 = 32,
    Float64 = 64,
    Float128 = 128,
}

impl FloatingSize {
    pub const VALUES: [FloatingSize; 4] = [
        FloatingSize::Float16,
        FloatingSize::Float32,
        FloatingSize::Float64,
        FloatingSize::Float128,
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub size: FloatingSize,
}

impl Float {
    pub fn new(size: FloatingSize) -> DataType {
        DataType::Float(Float { size })
    }

    pub fn from(src: &String) -> Option<Float> {
        if &src[0..5] == "float" {
            for size in FloatingSize::VALUES {
                if (size as u32).to_string() == &src[5..] {
                    return Some(Float { size });
                }
            }
        }

        None
    }

    pub fn eq(&self, x: &Float) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("float{}", self.size as u32)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool {}

impl Bool {
    pub fn new() -> DataType {
        DataType::Bool(Bool {})
    }

    pub fn from(src: &String) -> Option<Bool> {
        if src == "bool" {
            Some(Bool {})
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        "bool".to_owned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub typ: Box<DataType>,
}

impl Array {
    pub fn new(typ: Box<DataType>) -> DataType {
        DataType::Array(Array { typ })
    }

    pub fn from(src: &String) -> Option<Array> {
        if &src[src.len() - 2..src.len()] == "[]" {
            println!("{}", &src[0..src.len() - 2]);
        }

        // TODO: Implement
        todo!()
    }

    pub fn eq(&self, x: &Array) -> bool {
        self.typ.eq(&x.typ)
    }

    pub fn to_string(&self) -> String {
        format!("{}[]", self.typ.to_string())
    }
}
