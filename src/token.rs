/// Lisp式を表す列挙型
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// 整数
    Int(i32),
    /// シンボル
    Symbol(String),
    /// リスト
    List(Vec<Expr>),
    /// Lambda式
    Lambda(Vec<String>, Box<Expr>),
}
