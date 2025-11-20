use std::fmt;

pub trait Expression: fmt::Debug {}

pub trait Atom {
    type Val;

    fn val(&self) -> &Self::Val;
}

#[derive(Debug)]
pub struct IntAtom {
    pub n: i32,
}

impl Expression for IntAtom {}

impl Atom for IntAtom {
    type Val = i32;

    fn val(&self) -> &Self::Val {
        &self.n
    }
}

#[derive(Debug)]
pub struct SymbolAtom {
    pub name: String,
}

impl Expression for SymbolAtom {}

impl Atom for SymbolAtom {
    type Val = String;

    fn val(&self) -> &Self::Val {
        &self.name
    }
}

#[derive(Debug)]
pub struct ListExpression {
    pub elems: Vec<Box<dyn Expression>>,
}

impl Expression for ListExpression {}
