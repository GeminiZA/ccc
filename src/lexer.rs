use crate::token::Token;

#[derive(Debug)]
pub enum LexError {
    ExpectedToken,
    NotImplemented,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let mut cur_token_string = String::new();

    let break_chars = " \t\n{}();-~!+*/<>&|=";
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
            } else if c == '{' {
                tokens.push(Token::OpenBrace);
            } else if c == '}' {
                tokens.push(Token::CloseBrace);
            } else if c == '(' {
                tokens.push(Token::OpenParen);
            } else if c == ')' {
                tokens.push(Token::CloseParen);
            } else if c == ';' {
                tokens.push(Token::SemiColon);
            } else if c == '-' {
                tokens.push(Token::OperatorMinus);
            } else if c == '~' {
                tokens.push(Token::OperatorComplement);
            } else if c == '!' {
                match c_i.peek() {
                    Some('=') => {
                        tokens.push(Token::OperatorNotEqual);
                        c_i.next();
                    }
                    _ => tokens.push(Token::OperatorNegation),
                }
            } else if c == '+' {
                tokens.push(Token::OperatorAddtion);
            } else if c == '*' {
                tokens.push(Token::OperatorMultiplication);
            } else if c == '/' {
                tokens.push(Token::OperatorDivision);
            } else if c == '&' {
                match c_i.peek() {
                    Some('&') => {
                        tokens.push(Token::OperatorAnd);
                        c_i.next();
                    }
                    _ => return Err(LexError::NotImplemented),
                }
            } else if c == '<' {
                match c_i.peek() {
                    Some('=') => {
                        tokens.push(Token::OperatorLessOrEqual);
                        c_i.next();
                    }
                    _ => tokens.push(Token::OperatorLess),
                }
            } else if c == '>' {
                match c_i.peek() {
                    Some('=') => {
                        tokens.push(Token::OperatorGreaterOrEqual);
                        c_i.next();
                    }
                    _ => tokens.push(Token::OperatorGreater),
                }
            } else if c == '|' {
                match c_i.peek() {
                    Some('|') => {
                        tokens.push(Token::OperatorOr);
                        c_i.next();
                    }
                    _ => return Err(LexError::NotImplemented),
                }
            } else if c == '=' {
                match c_i.peek() {
                    Some('=') => {
                        tokens.push(Token::OperatorEqual);
                        c_i.next();
                    }
                    _ => return Err(LexError::NotImplemented),
                }
            }
        } else {
            cur_token_string.push(c);
        }
    }
    return Ok(tokens);
}
