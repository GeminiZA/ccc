use crate::{token, Token};
use core::slice::Iter;
use std::ops::Mul;

#[derive(Debug)]
pub enum InFunction {
    ParseProgram,
    ParseFunction,
    ParseStatement,
    ParseExpression,
    ParseLogicalAndExpression,
    ParseEqualityExpression,
    ParseRelationalExpression,
    ParseAdditiveExpression,
    ParseTerm,
    ParseFactor,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token, InFunction),
    ExpectedToken,
    Undefined,
}

#[derive(Debug)]
pub struct Program {
    // <program> ::= <function>
    pub m_function: Function,
}

#[derive(Debug)]
pub enum FunctionType {
    Int,
}

#[derive(Debug)]
pub struct Function {
    // <function> ::= "int" <id> "(" ")" "{" <statement> "}"
    pub m_type: FunctionType,
    pub m_id: String,
    pub m_statement: Option<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    // <statement> ::= "return" <exp> ";"
    Return(Expression),
    // Assign { name: String, value: Expression },
    // Other types if etc
}

#[derive(Debug)]
pub struct Expression {
    // <exp> ::= <logical-and-exp> { "||" <logical-and-exp> }
    pub m_first: Box<LogicalAndExpression>,
    pub m_rest: Vec<LogicalAndExpression>,
}

#[derive(Debug)]
pub struct LogicalAndExpression {
    // <logical-and-exp> ::= <equality-exp> { "&&" <equality-exp> }
    pub m_first: Box<EqualityExpression>,
    pub m_rest: Vec<(EqualityExpression)>,
}

#[derive(Debug)]
pub enum EqualityOperator {
    NotEqual,
    Equal,
}

#[derive(Debug)]
pub struct EqualityExpression {
    // <equality-exp> ::= <relational-exp> { ("!=" | "==") <relational-exp> }
    pub m_first: Box<RelationalExpression>,
    pub m_rest: Vec<(EqualityOperator, RelationalExpression)>,
}

#[derive(Debug)]
pub enum RelationalOperator {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Debug)]
pub struct RelationalExpression {
    // <relational-exp> ::= <additive-exp> { ("<" | ">" | "<=" | ">=") <additive-exp> }
    pub m_first: Box<AdditiveExpression>,
    pub m_rest: Vec<(RelationalOperator, AdditiveExpression)>,
}

#[derive(Debug)]
pub enum AdditiveOperator {
    Minus,
    Addition,
}

#[derive(Debug)]
pub struct AdditiveExpression {
    // <additive-exp> ::= <term> { ("+" | "-") <term> }
    // has operators + -
    pub m_first_term: Box<Term>,
    pub m_rest: Vec<(AdditiveOperator, Term)>,
}

#[derive(Debug)]
pub enum MultiplicativeOperator {
    Multiplication,
    Division,
}

#[derive(Debug)]
pub struct Term {
    // <term> ::= <factor> { ("*" | "/") <factor> }
    // Has operators * /
    pub m_first_factor: Box<Factor>,
    pub m_rest: Vec<(MultiplicativeOperator, Factor)>,
}

#[derive(Debug)]
pub enum UnaryOperator {
    // <unary_op> ::= "!" | "~" | "-"
    Complement,
    Negation,
    Minus,
}

#[derive(Debug)]
pub enum Factor {
    // <factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
    Constant { m_value: i32 },
    UnaryOperation { m_opertator: UnaryOperator, m_factor: Box<Factor> },
    Braced { m_expression: Expression },
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
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
    };

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    function.m_id = match token {
        Token::Identifier(s) => s.clone(),
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
    };

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::OpenParen => (),
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
    }

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::CloseParen => (),
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
    }

    token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::OpenBrace => (),
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
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
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
    }

    return Ok(function);
}

fn parse_statement(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Statement, ParseError> {
    // println!("Parsing statement from {:?}", &token_iter);
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
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseStatement,
            ))
        }
    }

    let mut token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };
    match token {
        Token::SemiColon => return Ok(statement),
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseStatement,
            ))
        }
    }
}

fn parse_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Expression, ParseError> {
    // println!("Parsing Expression from {:?}", &token_iter);
    let mut expression = match parse_logical_and_expression(token_iter) {
        Ok(l_a_e) => {
            Expression { m_first: Box::new(l_a_e), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorOr => {
                token_iter.next();
                expression.m_rest.push(
                    match parse_logical_and_expression(token_iter) {
                        Ok(l_a_e) => l_a_e,
                        Err(e) => return Err(e),
                    },
                )
            }
            _ => break,
        }
    }
    return Ok(expression);
}

fn parse_logical_and_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<LogicalAndExpression, ParseError> {
    // println!("Parsing Logical And Expression from {:?}", &token_iter);
    let mut local_and_expression = match parse_equality_expression(token_iter) {
        Ok(e_e) => {
            LogicalAndExpression { m_first: Box::new(e_e), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorAnd => {
                token_iter.next();
                local_and_expression.m_rest.push(
                    match parse_equality_expression(token_iter) {
                        Ok(e_e) => e_e,
                        Err(e) => return Err(e),
                    },
                )
            }
            _ => break,
        }
    }

    return Ok(local_and_expression);
}

fn parse_equality_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<EqualityExpression, ParseError> {
    // println!("Parsing Equality Expression from {:?}", &token_iter);
    let mut equality_expression = match parse_relational_expression(token_iter)
    {
        Ok(r_e) => {
            EqualityExpression { m_first: Box::new(r_e), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorNotEqual => {
                token_iter.next();
                equality_expression.m_rest.push((
                    EqualityOperator::NotEqual,
                    match parse_relational_expression(token_iter) {
                        Ok(r_e) => r_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorEqual => {
                token_iter.next();
                equality_expression.m_rest.push((
                    EqualityOperator::Equal,
                    match parse_relational_expression(token_iter) {
                        Ok(r_e) => r_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            _ => break,
        }
    }

    return Ok(equality_expression);
}

fn parse_relational_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<RelationalExpression, ParseError> {
    // println!("Parsing Relational Expression from {:?}", &token_iter);
    let mut relational_expression = match parse_additive_expression(token_iter)
    {
        Ok(a_e) => {
            RelationalExpression { m_first: Box::new(a_e), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorLess => {
                token_iter.next();
                relational_expression.m_rest.push((
                    RelationalOperator::Less,
                    match parse_additive_expression(token_iter) {
                        Ok(a_e) => a_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorLessOrEqual => {
                token_iter.next();
                relational_expression.m_rest.push((
                    RelationalOperator::LessOrEqual,
                    match parse_additive_expression(token_iter) {
                        Ok(a_e) => a_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorGreater => {
                token_iter.next();
                relational_expression.m_rest.push((
                    RelationalOperator::Greater,
                    match parse_additive_expression(token_iter) {
                        Ok(a_e) => a_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::OperatorGreaterOrEqual => {
                token_iter.next();
                relational_expression.m_rest.push((
                    RelationalOperator::GreaterOrEqual,
                    match parse_additive_expression(token_iter) {
                        Ok(a_e) => a_e,
                        Err(e) => return Err(e),
                    },
                ))
            }
            _ => break,
        }
    }

    return Ok(relational_expression);
}

fn parse_additive_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<AdditiveExpression, ParseError> {
    // println!("Parsing Additive Expression from {:?}", &token_iter);
    //Members
    // // println!("Parsing Expression from {:?}", &token_iter);
    let mut additive_expression = match parse_term(token_iter) {
        Ok(t) => {
            AdditiveExpression { m_first_term: Box::new(t), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        // // println!("next token: {:?}", &next);
        match next {
            Token::OperatorAddtion => {
                token_iter.next();
                additive_expression.m_rest.push((
                    AdditiveOperator::Addition,
                    match parse_term(token_iter) {
                        Ok(term) => term,
                        Err(e) => return Err(e),
                    },
                ));
            }
            Token::OperatorMinus => {
                token_iter.next();
                additive_expression.m_rest.push((
                    AdditiveOperator::Minus,
                    match parse_term(token_iter) {
                        Ok(term) => term,
                        Err(e) => return Err(e),
                    },
                ));
            }
            _ => break,
        }
    }

    return Ok(additive_expression);
}

fn parse_term(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Term, ParseError> {
    // println!("Parsing term from {:?}", &token_iter);
    let mut term = match parse_factor(token_iter) {
        Ok(f) => Term { m_first_factor: Box::new(f), m_rest: Vec::new() },
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorMultiplication => {
                token_iter.next();
                term.m_rest.push((
                    MultiplicativeOperator::Multiplication,
                    match parse_factor(token_iter) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    },
                ));
            }
            Token::OperatorDivision => {
                token_iter.next();
                term.m_rest.push((
                    MultiplicativeOperator::Division,
                    match parse_factor(token_iter) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    },
                ));
            }
            _ => break,
        }
    }

    // println!("returning term {:?}", term);

    return Ok(term);
}

fn parse_factor(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Factor, ParseError> {
    // println!("Parsing factor from {:?}", &token_iter);
    let mut factor: Factor;
    let mut cur_token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    match cur_token {
        Token::OpenParen => {
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
                Token::CloseParen => (),
                t => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseFactor,
                    ))
                }
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
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFactor,
            ))
        }
    }

    // println!("returning factor {:?}", factor);

    return Ok(factor);
}
