use crate::Token;
use core::slice::Iter;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedToken,
    Undefined,
}

#[derive(Debug)]
pub struct Program {
    pub m_function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub m_id: String,
    pub m_statement: Statement,
}

#[derive(Debug)]
pub enum Expression {
    Arithmetic { m_value: i32 },
    String { m_value: String },
}

// Todo change function, expression and statement to enums to support different statement types etc
// #[derive(Debug)]
// pub enum Statement {
// Return(Expression),
// Assign { name: String, value: Expression },
// // other types of statements...
// }
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Assign { name: String, value: Expression },
    // Other types if etc
}

pub fn parse_program(tokens: &Vec<Token>) -> Result<Program, ParseError> {
    let mut token_iter = tokens.iter();
    let mut function;

    match parse_function(&mut token_iter) {
        Ok(func) => function = func,
        Err(e) => return Err(e),
    }

    return Ok(Program {
        m_function: function,
    });
}

fn parse_function(token_iter: &mut Iter<Token>) -> Result<Function, ParseError> {
    //Members
    let statement;
    let id;
    //Token Iterator
    let mut token = token_iter.next();
    match token {
        Some(Token::KeywordInt) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    token = token_iter.next();
    match token {
        Some(Token::Identifier(s)) => id = s.clone(),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    token = token_iter.next();
    match token {
        Some(Token::OpenParen) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    token = token_iter.next();
    match token {
        Some(Token::CloseParen) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    token = token_iter.next();
    match token {
        Some(Token::OpenBrace) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    match parse_statement(token_iter) {
        Ok(s) => statement = s,
        Err(e) => return Err(e),
    }

    token = token_iter.next();
    match token {
        Some(Token::CloseBrace) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    return Ok(Function {
        m_statement: statement,
        m_id: id,
    });
}

fn parse_statement(token_iter: &mut Iter<Token>) -> Result<Statement, ParseError> {
    //Members
    let expression;
    //Token Iter
    let mut token = token_iter.next();

    match token {
        Some(Token::KeywordReturn) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    match parse_expression(token_iter) {
        Ok(e) => expression = e,
        Err(e) => return Err(e),
    }

    token = token_iter.next();
    match token {
        Some(Token::SemiColon) => (),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    return Ok(Statement::Return(expression));
}

fn parse_expression(token_iter: &mut Iter<Token>) -> Result<Expression, ParseError> {
    //Members
    let value: i32;

    let mut token = token_iter.next();
    match token {
        Some(Token::IntLiteral(val)) => value = val.clone(),
        Some(_) => return Err(ParseError::UnexpectedToken),
        None => return Err(ParseError::ExpectedToken),
    }

    return Ok(Expression::Arithmetic { m_value: (value) });
}
