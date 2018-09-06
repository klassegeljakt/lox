#![allow(non_snake_case)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::*};

#[derive(Parser)]
#[grammar = "lox.pest"]
struct Lox;

fn main() {
  let source = "fun main() { }";
  Lox::parse(Rule::Program, source)
    .unwrap()
    .for_each(|pair| {
      pair.into_inner().for_each(|inner_pair| {
        let inner_span = inner_pair.clone().into_span();
        println!("{}: {}", inner_pair, inner_span.as_str());
    })
  });
}

struct AST {
  decls: Vec<Decl>
}
impl AST {
  fn from(source: &str) -> Self {
    Self {
      decls: Lox::parse(Rule::Program, source)
        .unwrap()
        .map(|pair| Decl::from(pair))
        .into_iter()
        .collect()
    }
  }
}

enum Decl {
  Class(Class),
  Fun(Fun),
  Var(Var),
  Stmt(Stmt),
}

impl Decl {
  fn from(pair: Pair<Rule>) -> Self {
    Decl(
      match pair.as_rule() {
        Rule::Class => Class::from(pair),
        Rule::Fun   => Class::from(pair),
        Rule::Var   => Class::from(pair),
        Rule::Stmt  => Class::from(pair),
      }
    )
  }
}

struct Class {
  fields: Vec<Field>,
  methods: Vec<Fun>,
}

impl Class {
  fn from(pair: Pair<Rule>) -> Self {
    let (fields, methods) = pair.into_inner()
    Self {
      fields: 
      methods: 
    }
  }
}

struct Field {
  
}

struct Fun {
  params: Vec<Param>,
  body: Box<Expr>,
}

struct Param {
  id: Id, 
}

struct Var {
  id: Id,
  val: Expr,
}

enum Stmt {
  ExprStmt(Expr),
  For(For),
  If(If),
  Print(Print),
  Return(Return),
  While(While),
  Block(Expr),
}

struct For {
  init: Option<Expr>,
  cond: Option<Expr>,
  after: Option<Expr>,
  body: Expr,
}

struct If {
  cond: Expr,
  then_branch: Box<Stmt>,
  else_branch: Option<Box<Stmt>>,
}

struct Print {
  expr: Expr,
}

struct Return {
  expr: Expr,
}

struct While {
  cond: Expr,
  body: Box<Stmt>,
}

enum Expr {
  Assign(Assign),
  Binary(Binary),
  Unary(Unary),
  Call(Call),
  Primary(Primary),
}

struct Call {
  expr: Primary,
  methods: Vec<Method>,
}

struct Method {
  id: Id,
  
}

struct Assign {
  lval: Box<Call>,
  rval: Box<Expr>,
}


struct Binary {
  l: Box<Expr>,
  op: BinaryOp,
  r: Box<Expr>,
}

enum BinaryOp {
  Or,  And, Eq,  Neq,
  Gt,  Geq, Lt,  Leq,
  Add, Sub, Mul, Div,
}

struct Unary {
  op: UnaryOp,
  expr: Box<Expr>
}

enum UnaryOp {
  Not,
  Neg,
}

struct Access {
  
}

enum Primary {
  Expr(Box<Expr>),
  Lit(Lit),
  Id(Id),
  Super(Id),
}

type Id = String;

enum Lit {
  Float(f32),
  Int(i32),
  Str(String),
  Char(char),
  Bool(bool),
}

