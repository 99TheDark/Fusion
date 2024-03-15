#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int(Int),
    Uint(Uint),
    Float(Float),
    Bool(Bool),
    Array(Array),
}

impl DataType {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Int {
    pub size: Option<u32>,
}

impl Int {
    pub fn new(size: Option<u32>) -> DataType {
        DataType::Int(Int { size })
    }

    pub fn eq(&self, x: &Int) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        match self.size {
            Some(size) => format!("int{}", size),
            None => "int".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Uint {
    pub size: Option<u32>,
}

impl Uint {
    pub fn new(size: Option<u32>) -> DataType {
        DataType::Uint(Uint { size })
    }

    pub fn eq(&self, x: &Uint) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        match self.size {
            Some(size) => format!("uint{}", size),
            None => "uint".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub size: Option<u32>,
}

impl Float {
    pub fn new(size: Option<u32>) -> DataType {
        DataType::Float(Float { size })
    }

    // Half-implemented, to work more on later :P
    pub fn from(src: String) -> Option<Float> {
        if &src[0..5] == "float" {
            println!("{}", &src[6..]);
            Some(Float { size: Some(32) })
        } else {
            None
        }
    }

    pub fn eq(&self, x: &Float) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        match self.size {
            Some(size) => format!("float{}", size),
            None => "float".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool {}

impl Bool {
    pub fn new() -> DataType {
        DataType::Bool(Bool {})
    }

    pub fn from(src: String) -> Option<Bool> {
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
    pub typ: Option<Box<DataType>>,
}

impl Array {
    pub fn new(typ: Option<Box<DataType>>) -> DataType {
        DataType::Array(Array { typ })
    }

    pub fn eq(&self, x: &Array) -> bool {
        self.typ.eq(&x.typ)
    }

    pub fn to_string(&self) -> String {
        match &self.typ {
            Some(typ) => format!("{}[]", typ.to_string()),
            None => "[]".to_owned(),
        }
    }
}
