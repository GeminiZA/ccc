use crate::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut cur_token_string = String::new();

    let break_chars = " \t\n{}();";
    let white_space = " \t\n";

    for c in input.chars() {
        if break_chars.contains(c) {
            // Lex the cur_token_string then lex the current char (break chars)
            if cur_token_string.len() > 0 {
                if cur_token_string == "return" {
                    // return keyword
                    tokens.push(Token::KeywordReturn);
                } else if cur_token_string == "int" {
                    // int keyword
                    tokens.push(Token::KeywordInt);
                } else {
                    // try parse to int then its an int literal
                    if let Ok(i) = cur_token_string.parse::<i32>() {
                        tokens.push(Token::IntLiteral(i));
                    } else {
                        tokens.push(Token::Identifier(cur_token_string.clone()));
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
            }
        } else {
            cur_token_string.push(c);
        }
    }
    return tokens;
}
