extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use pest::{Parser, prec_climber::*, iterators::*};

#[derive(Parser)]
#[grammar = "arith.pest"]
struct Arith;
trait Parse {
  fn from(pair: Pair<Rule>) -> Self;
}

#[derive(Debug)]
enum Expr {
  Num(String),
  Add(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
}

type Id = String;

struct Type {
  id: String
}

struct Arg {
  id: Id,
  ty: Type,
}

impl Parse for Arg {
  fn from(pair: Pair<Rule>) -> Self {
    
  }
}

struct Fun {
  args: Vec<Arg>,
  body: Expr
}

impl Fun {
  fn from(pair: Pair<Rule>) -> Self {
    let mut args = Vec::new();
    let mut block = Vec::new();
    pair.into_inner()
      .for_each(|node|
        match node.as_rule() {
          Rule::Arg => args.append(Arg::),
          Rule::Block => parse_primary(node),
          _ => unreachable!(),
        });
    let inner = pair.into_inner();
    Fun {
      args: inner.get
        
      body: parse_expr(pair)
    }
  }
}

enum Decl {
  Fun(Fun)
}

impl Decl {

}

lazy_static! {
  static ref ARITH_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
      Operator::new(Rule::Add, Assoc::Left),
      Operator::new(Rule::Mul, Assoc::Left),
  ]);
}

fn infix(lhs: Expr, op: Pair<Rule>, rhs: Expr) -> Expr {
  match op.as_rule() {
    Rule::Add => Expr::Add(Box::new(lhs), Box::new(rhs)),
    Rule::Mul => Expr::Mul(Box::new(lhs), Box::new(rhs)),
    _ => unreachable!()
  }
}

struct AST {
  decls: Vec<Decl>
}


fn parse_decl(pair: Pair<Rule>) -> Decl {
  match self.as_rule() {
    Rule::Fun => Decl::Fun(self.parse_fun()),
    _ => unreachable!()
  }
}

fn parse_primary(pair: Pair<Rule>) -> Expr {
  match self.as_rule() {
    Rule::Num => Expr::Num("a".to_owned()),
    _ => unreachable!()
  }
}

impl AST {
  fn from(input: &str) -> AST {
    Self {
      decls: Arith::parse(Rule::Main, input)
        .unwrap()
        .map(|node| parse_decl(node))
        .collect()
    }
  }
}

fn main() {
  let input = "fn() {10 * 22 + 34}";
  AST::from(input);
}

    //let result = ARITH_CLIMBER.climb(pairs, primary, infix);
    //println!("{:?}", result);
