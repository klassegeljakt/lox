extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ident.pest"]
struct MyParser;

fn main() {
  println!("{:?}", MyParser::parse(Rule::main, "a+1").unwrap());
}
