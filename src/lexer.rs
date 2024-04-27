use crate::token::Token;

#[derive(Debug)]
pub enum LexError {
    ExpectedToken,
    NotImplemented,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let mut cur_token_string = String::new();

    let break_chars = " \t\n{}();-~!+*/%<>&|=:?,";
    let white_space = " \t\n";

    let mut i = 0;
    let mut c_i = input.chars().peekable();
    while let Some(c) = c_i.next() {
        if break_chars.contains(c) {
            // Lex the cur_token_string then lex the current char (break chars)
            if cur_token_string.len() > 0 {
                if cur_token_string == "return" {
                    tokens.push(Token::KeywordReturn);
                } else if cur_token_string == "int" {
                    tokens.push(Token::KeywordInt);
                } else if cur_token_string == "if" {
                    tokens.push(Token::KeywordIf);
                } else if cur_token_string == "else" {
                    tokens.push(Token::KeywordElse);
                } else if cur_token_string == "for" {
                    tokens.push(Token::KeywordFor);
                } else if cur_token_string == "while" {
                    tokens.push(Token::KeywordWhile);
                } else if cur_token_string == "do" {
                    tokens.push(Token::KeywordDo);
                } else if cur_token_string == "break" {
                    tokens.push(Token::KeywordBreak);
                } else if cur_token_string == "continue" {
                    tokens.push(Token::KeywordContinue);
                } else {
                    // try parse to int then its an int literal
                    if let Ok(i) = cur_token_string.parse::<i32>() {
                        tokens.push(Token::IntLiteral(i));
                    } else {
                        tokens
                            .push(Token::Identifier(cur_token_string.clone()));
                    } // TODO: Add other literals
                }
            }
            cur_token_string.clear();
            // Lex the currect char (break chars)
            if white_space.contains(c) {
                continue;
            } else {
                match c {
                    '{' => tokens.push(Token::OpenBrace),
                    '}' => tokens.push(Token::CloseBrace),
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    ';' => tokens.push(Token::SemiColon),
                    '-' => tokens.push(Token::OperatorMinus),
                    '~' => tokens.push(Token::OperatorComplement),
                    '!' => match c_i.peek() {
                        Some('=') => {
                            tokens.push(Token::OperatorNotEqual);
                            c_i.next();
                        }
                        _ => tokens.push(Token::OperatorNegation),
                    },
                    '+' => tokens.push(Token::OperatorAddtion),
                    '*' => tokens.push(Token::OperatorMultiplication),
                    '/' => tokens.push(Token::OperatorDivision),
                    '%' => tokens.push(Token::OperatorModulo),
                    '&' => match c_i.peek() {
                        Some('&') => {
                            tokens.push(Token::OperatorAnd);
                            c_i.next();
                        }
                        _ => return Err(LexError::NotImplemented),
                    },
                    '<' => match c_i.peek() {
                        Some('=') => {
                            tokens.push(Token::OperatorLessOrEqual);
                            c_i.next();
                        }
                        _ => tokens.push(Token::OperatorLess),
                    },
                    '>' => match c_i.peek() {
                        Some('=') => {
                            tokens.push(Token::OperatorGreaterOrEqual);
                            c_i.next();
                        }
                        _ => tokens.push(Token::OperatorGreater),
                    },
                    '|' => match c_i.peek() {
                        Some('|') => {
                            tokens.push(Token::OperatorOr);
                            c_i.next();
                        }
                        _ => return Err(LexError::NotImplemented),
                    },
                    '=' => match c_i.peek() {
                        Some('=') => {
                            tokens.push(Token::OperatorEqual);
                            c_i.next();
                        }
                        _ => tokens.push(Token::OperatorAssign),
                    },
                    ':' => tokens.push(Token::Colon),
                    '?' => tokens.push(Token::QuestionMark),
                    ',' => tokens.push(Token::Comma),
                    _ => return Err(LexError::NotImplemented),
                }
            }
        } else {
            cur_token_string.push(c);
        }
    }
    tokens.push(Token::EndOfFile);
    return Ok(tokens);
}
