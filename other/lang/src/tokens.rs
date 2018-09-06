
#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens.
  LPAREN, RPAREN, LBRACE, RBRACE,
  COMMA, DOT, MINUS, PLUS, SEMI, SLASH, STAR,

  // One or two character tokens.
  EX, EXEQ,
  EQ, EQEQ,
  GT, GTEQ,
  LT, LTEQ,

  // Literals.
  ID(String), STR(String), NUM(i32),

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

pub struct Token {
  tt: TokenType,
  line: usize,
}

impl Token {
  pub fn new(tt: TokenType, line: usize) -> Self {
    Self { tt: tt, line: line }
  }
}

impl ToString for Token {
    fn to_string(&self) -> String {
    format!("{:?} {}", self.tt, self.line)
  }
}
