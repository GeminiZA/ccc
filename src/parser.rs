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
pub enum FunctionType {
    Int,
}

#[derive(Debug)]
pub struct Function {
    pub m_type: FunctionType,
    pub m_id: String,
    pub m_statement: Option<Statement>,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negation,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Minus,
    Addition,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub struct Expression {
    // has operators + -
    pub m_first_term: Box<Term>,
    pub m_rest: Vec<(BinaryOperator, Term)>,
}

#[derive(Debug)]
pub struct Term {
    // Has operators * /
    pub m_first_factor: Box<Factor>,
    pub m_rest: Vec<(BinaryOperator, Factor)>,
}

#[derive(Debug)]
pub enum Factor {
    Constant { m_value: i32 },
    UnaryOperation { m_opertator: UnaryOperator, m_factor: Box<Factor> },
    Braced { m_expression: Expression },
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    // Assign { name: String, value: Expression },
    // Other types if etc
}

pub fn parse_program(tokens: &Vec<Token>) -> Result<Program, ParseError> {
    let mut token_iter = tokens.iter().peekable();

    let function = match parse_function(&mut token_iter) {
        Ok(func) => func,
        Err(e) => return Err(e),
    };

    return Ok(Program { m_function: function });
}

fn parse_function(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Function, ParseError> {
    //Token Iterator
    let mut token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    let mut function = match token {
        Token::KeywordInt => Function {
            m_type: FunctionType::Int,
            m_id: String::new(),
            m_statement: None,
        },
        _ => return Err(ParseError::UnexpectedToken),
    };

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    function.m_id = match token {
        Token::Identifier(s) => s.clone(),
        _ => return Err(ParseError::UnexpectedToken),
    };

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::OpenParen => (),
        _ => return Err(ParseError::UnexpectedToken),
    }

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::CloseParen => (),
        _ => return Err(ParseError::UnexpectedToken),
    }

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::OpenBrace => (),
        _ => return Err(ParseError::UnexpectedToken),
    }

    function.m_statement = match parse_statement(token_iter) {
        Ok(s) => Some(s),
        Err(e) => return Err(e),
    };

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::CloseBrace => (),
        _ => return Err(ParseError::UnexpectedToken),
    }

    return Ok(function);
}

fn parse_statement(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Statement, ParseError> {
    //Members
    let mut statement: Statement;
    //Token Iter
    let mut token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    match token {
        Token::KeywordReturn => {
            let expression = match parse_expression(token_iter) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };
            statement = Statement::Return(expression);
        }
        _ => return Err(ParseError::UnexpectedToken),
    }

    let mut token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::SemiColon => return Ok(statement),
        _ => return Err(ParseError::UnexpectedToken),
    }
}

fn parse_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Expression, ParseError> {
    //Members
    let mut expression = match parse_term(token_iter) {
        Ok(t) => Expression { m_first_term: Box::new(t), m_rest: Vec::new() },
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorAddtion => {
                token_iter.next();
                expression.m_rest.push((
                    BinaryOperator::Addition,
                    match parse_term(token_iter) {
                        Ok(term) => term,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorMinus => {
                token_iter.next();
                expression.m_rest.push((
                    BinaryOperator::Minus,
                    match parse_term(token_iter) {
                        Ok(term) => term,
                        Err(e) => return Err(e),
                    },
                ))
            }
            _ => break,
        }
    }

    return Ok(expression);
}

fn parse_term(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Term, ParseError> {
    let mut term = match parse_factor(token_iter) {
        Ok(f) => Term { m_first_factor: Box::new(f), m_rest: Vec::new() },
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorMultiplication => {
                token_iter.next();
                term.m_rest.push((
                    BinaryOperator::Multiplication,
                    match parse_factor(token_iter) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorDivision => {
                token_iter.next();
                term.m_rest.push((
                    BinaryOperator::Division,
                    match parse_factor(token_iter) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    },
                ))
            }
            _ => break,
        }
    }

    return Ok(term);
}

fn parse_factor(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Factor, ParseError> {
    let mut factor: Factor;
    let mut cur_token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    match cur_token {
        Token::OpenParen => {
            token_iter.next();
            factor = Factor::Braced {
                m_expression: match parse_expression(token_iter) {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                },
            };
            let next_token = match token_iter.next() {
                Some(t) => t,
                None => return Err(ParseError::ExpectedToken),
            };
            match next_token {
                Token::CloseBrace => (),
                _ => return Err(ParseError::UnexpectedToken),
            }
        }
        Token::OperatorNegation => {
            let next_factor = match parse_factor(token_iter) {
                Ok(f) => Box::new(f),
                Err(e) => return Err(e),
            };
            factor = Factor::UnaryOperation {
                m_opertator: UnaryOperator::Negation,
                m_factor: next_factor,
            };
        }
        Token::OperatorComplement => {
            let next_factor = match parse_factor(token_iter) {
                Ok(f) => Box::new(f),
                Err(e) => return Err(e),
            };
            factor = Factor::UnaryOperation {
                m_opertator: UnaryOperator::Complement,
                m_factor: next_factor,
            };
        }
        Token::OperatorMinus => {
            let next_factor = match parse_factor(token_iter) {
                Ok(f) => Box::new(f),
                Err(e) => return Err(e),
            };
            factor = Factor::UnaryOperation {
                m_opertator: UnaryOperator::Minus,
                m_factor: next_factor,
            };
        }
        Token::IntLiteral(val) => {
            factor = Factor::Constant { m_value: val.clone() }
        }
        _ => return Err(ParseError::UnexpectedToken),
    }

    return Ok(factor);
}
