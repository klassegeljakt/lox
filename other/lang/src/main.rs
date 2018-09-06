mod tokens;
mod scanner;
mod symbols;

#[macro_use]
extern crate derive_more;

use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::prelude::Read;
use std::io;
use std::io::BufRead;
use scanner::Scanner;


fn run(source: String) -> bool {
  let scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens();
  for token in tokens {
    println!("{}", token);
  };
  true
}

fn error(line: i32, message: String) {
  report(line, "", message);
}

fn report(line: i32, at: &str, message: String) {
  println!("[line {}] Error{}: {}", line, at, message);
}

fn run_file(path: String) {
  let file = File::open(path).expect("File not found");
  let mut source = String::new();
  file.read_to_string(&mut source).expect("Could not read file");
  let error = run(source);
  if error {
    exit(65);
  }
}

fn run_prompt() -> io::Result<()> {
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();
  loop {
    print!("> ");
    handle.read_line(&mut buffer);
    run(buffer);
  }
}


fn main() {
  let args = args();
  if args.len() > 1 {
    println!("Usage: jlox [script]");
    exit(64);
  } else if args.len() == 1  {
    run_file(args.next());
  } else {
    run_prompt();
  }
}
