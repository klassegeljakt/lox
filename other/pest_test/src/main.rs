extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::prec_climber;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("ident.pest");

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;


fn main() { }
