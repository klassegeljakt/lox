use tokens::TokenType::*;
use tokens::TokenType;
use tokens::Token;
use std::str::Chars;
use std::iter::Peekable;
use std::ops::Range;

pub struct Scanner<'a> {
  source: String,
  chars: Peekable<Chars<'a>>,
  tokens: Vec<Token>,
  beg: usize,
  end: usize,
  len: usize,
  line: usize,
  log: Vec<(usize, String)>,
}

impl<'a> Scanner<'a> {

  fn new(source: String) -> Self {
    let chars = source.as_str().chars().peekable();
    Self {
      source: source,
      chars: chars,
      tokens: Vec::new(),
      beg: 0,
      len: chars.count(),
      end: 0,
      line: 1,
      log: Vec::new(),
    }
  }

  fn advance(&mut self) -> char {
    self.end += 1;
    self.chars.next().unwrap()
  }

  fn push(&mut self, tt: TokenType) {
    self.tokens.push(Token::new(tt, self.line));
  }

  fn slice(&mut self, range: Range<usize>) -> String {
    self.source[range].to_owned()
  }

  fn log(&mut self, message: &str) {
    self.log.push((self.line, message.to_owned()))
  }

  fn next_is(&mut self, expected: char) -> bool {
    if self.eof() || self.peek() != expected  {
      false
    } else {
      self.end += 1;
      true
    }
  }

  fn peek(&mut self) -> char {
    *self.chars.peek().unwrap()
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      '(' => self.push(LPAREN),
      ')' => self.push(RPAREN),
      '{' => self.push(LBRACE),
      '}' => self.push(RBRACE),
      ',' => self.push(COMMA),
      '.' => self.push(DOT),
      '-' => self.push(MINUS),
      '+' => self.push(PLUS),
      ';' => self.push(SEMI),
      '*' => self.push(STAR),
      '!' => self.push(if self.next_is('=') { EXEQ } else { EX }),
      '=' => self.push(if self.next_is('=') { EQEQ } else { EQ }),
      '>' => self.push(if self.next_is('=') { LTEQ } else { LT }),
      '<' => self.push(if self.next_is('=') { GTEQ } else { GT }),
      '/' => self.slash(),
      '"' => self.string(),
      ' ' | '\r' | '\t' => {},
      c if is_numeric(c) => self.number(),
      c if is_alpha(c) => self.identifier(),
      '\n' => self.line += 1,
      _ => self.log("Unexpected token"),
    };
  }

  fn slash(&mut self) {
    if self.next_is('/') {
      while self.eof() && self.peek() != '\n' {
        self.advance();
      }
    } else {
      self.push(SLASH);
    }
  }

  fn string(&mut self) {
    while !self.eof() && self.peek() != '"' {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }
    if self.eof() {
      self.log("Unterminated string.");
    } else {
      self.advance();
      self.push(STR(self.slice(self.beg+1 .. self.end-1)));
    }
  }

  fn number(&mut self) {
    while is_numeric(self.peek()) {
      self.advance();
    }
    self.push(NUM(self.slice(self.beg .. self.end).parse::<i32>().unwrap()));
  }

  fn identifier(&mut self) {
    while is_alpha_numeric(self.peek()) {
      self.advance();
    }
    self.push(self.keyword().unwrap_or(ID(self.slice(self.beg..self.end))))
  }

  fn scan(&mut self) -> Vec<Token> {
    while !self.eof() {
      self.scan_token();
      self.beg = self.end;
    }
    self.push(EOF);
    self.tokens
  }

  fn eof(&mut self) -> bool {
    self.end >= self.len
  }

  fn keyword(&mut self) -> Option<TokenType> {
    match &self.source[self.beg..self.end] {
      "and"    => Some(AND),
      "class"  => Some(CLASS),
      "else"   => Some(ELSE),
      "false"  => Some(FALSE),
      "for"    => Some(FOR),
      "fun"    => Some(FUN),
      "if"     => Some(IF),
      "nil"    => Some(NIL),
      "or"     => Some(OR),
      "print"  => Some(PRINT),
      "return" => Some(RETURN),
      "super"  => Some(SUPER),
      "this"   => Some(THIS),
      "true"   => Some(TRUE),
      "var"    => Some(VAR),
      "while"  => Some(WHILE),
      _        => None,
    }
  } 

  
}

fn is_numeric(c: char) -> bool {
  match c {
    '0'...'9' => true,
    _ => false
  }
}

fn is_alpha(c: char) -> bool {
  match c {
    'A'...'Z' => true,
    'a'...'z' => true,
    _ => false,
  }
}

fn is_alpha_numeric(c: char) -> bool {
  is_alpha(c) || is_numeric(c)
}
