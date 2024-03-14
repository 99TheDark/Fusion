#[derive(Debug)]
pub enum DataType {
    Int(Int),
    Uint(Uint),
    Float(Float),
    Bool(Bool),
}

#[derive(Debug)]
pub struct Int {
    pub size: u32,
    pub val: i32,
}

#[derive(Debug)]
pub struct Uint {
    pub size: u32,
    pub val: u32,
}

#[derive(Debug)]
pub struct Float {
    pub size: u32,
    pub val: f32,
}

#[derive(Debug)]
pub struct Bool {
    pub size: u32,
    pub val: bool,
}
