#[derive(Debug)]
pub enum Token {
    KeywordInt,
    Identifier(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    KeywordReturn,
    IntLiteral(i32),
    SemiColon,
}

// // Incomplete debug formatter
//.impl fmt::Debug for Token {
//.fn fmt(&self, &mut fmt::Formatter) -> fmt::Result {
//.match self {
//.Token::KeywordInt => write!(f, "KeywordInt"),
//.Token::Identifier(id) => write!(f, "Identifier({})", id),
//.Token::OpenParen => write!(f, "OpenParen"),
//.Token::CloseParn => write!(f, "CloseParen"),
//.Token::OpenBrace => write!(f, "OpenBrace"),
//.Token::CloseBrace => write!(f, "CloseBrace"),
//.Token::KeywordReturn => write!(f, "KeywordReturn"),
//.Token::IntLiteral(i) => write!(f, "IntLiteral({})", i),
//.Token::SemiColon => write!(f, "SemiColon"),
//.}
//.}
//.}
