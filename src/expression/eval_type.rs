#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Bool,
    BoolArray(usize),
    Int,
    IntArray(usize),
    Float,
    FloatArray(usize),
    Str,
    StrArray(usize),
}
