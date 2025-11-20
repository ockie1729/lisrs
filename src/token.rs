use std::fmt;

/// Lisp式を表すトレイト
pub trait Expression: fmt::Debug {}

/// アトム値を取得するトレイト
pub trait Atom {
    type Val;

    fn val(&self) -> &Self::Val;
}

/// 整数アトム
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

/// シンボルアトム
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

/// リスト式
#[derive(Debug)]
pub struct ListExpression {
    pub elems: Vec<Box<dyn Expression>>,
}

impl Expression for ListExpression {}
