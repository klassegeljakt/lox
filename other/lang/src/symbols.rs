
use tokens::*;

pub enum Expr {
  Unary(Unary),
  Binary(Binary),
}

pub enum Literal {
  Num(i32),
  Str(String),
  False,
  True,
  Nil,
}

#[derive(Constructor)]
pub struct Unary {
  op: Token,
  v: Box<Expr>,
}

#[derive(Constructor)]
pub struct Binary {
  l: Box<Expr>,
  op: Token,
  r: Box<Expr>,
}

#[derive(Constructor)]
pub struct Grouping {
  l: Token,
  e: Box<Expr>,
  r: Token,
}


