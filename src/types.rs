#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int(Int),
    Uint(Uint),
    Float(Float),
    Bool(Bool),
    Array(Array),
}

impl DataType {
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
    pub size: u32,
    pub val: i32,
}

impl Int {
    pub fn eq(&self, x: Int) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("int{}", self.size)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Uint {
    pub size: u32,
    pub val: u32,
}

impl Uint {
    pub fn eq(&self, x: Uint) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("uint{}", self.size)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub size: u32,
    pub val: f32,
}

impl Float {
    pub fn from(src: String) -> Option<Float> {
        if &src[0..5] == "float" {
            println!("{}", &src[6..]);
            Some(Float { size: 32, val: 0.0 })
        } else {
            None
        }
    }

    pub fn eq(&self, x: Float) -> bool {
        self.size == x.size
    }

    pub fn to_string(&self) -> String {
        format!("float{}", self.size)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool {
    pub val: bool,
}

impl Bool {
    pub fn from(src: String) -> Option<Bool> {
        if src == "bool" {
            Some(Bool { val: false })
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
    pub fn to_string(&self) -> String {
        format!("{}[]", self.typ.to_string())
    }
}
