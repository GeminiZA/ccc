use crate::Token;
use core::slice::Iter;

const DEBUG: bool = false;

#[derive(Debug)]
pub enum InFunction {
    ParseProgram,
    ParseFunction,
    ParseBlockItem,
    ParseDeclaration,
    ParseStatement,
    ParseExpression,
    ParseConditionalExpression,
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
}

#[derive(Debug)]
pub struct Program {
    // <program> ::= <function>
    pub m_functions: Vec<Function>,
}

#[derive(Debug)]
pub enum FunctionType {
    Int,
}

#[derive(Debug)]
pub struct Function {
    // <function> ::= "int" <id> "(" ")" "{" <statement> "}"
    pub m_type: FunctionType,
    pub m_params: Vec<String>, // Change to struct when adding other types
    pub m_id: String,
    pub m_items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(Debug)]
pub enum Statement {
    // <statement> ::= "return" <exp> ";"
    // | <exp> ";"
    // | "int" <id> [ = <exp> ] ";"
    Return(Option<Expression>),
    Expression(Option<Expression>),
    If {
        m_condition: Expression,
        m_true_statement: Box<Statement>,
        m_else_statement: Option<Box<Statement>>,
    },
    Compound {
        m_block_items: Vec<Box<BlockItem>>,
    }, // Other types if etc
    For {
        m_inititial_expression: Option<Expression>,
        m_condition: Expression,
        m_post_expression: Option<Expression>,
        m_statement: Box<Statement>,
    },
    ForDecl {
        m_initial_declaration: Declaration,
        m_condition: Expression,
        m_post_expression: Option<Expression>,
        m_statement: Box<Statement>,
    },
    While {
        m_condition: Expression,
        m_statement: Box<Statement>,
    },
    Do {
        m_statement: Box<Statement>,
        m_condition: Expression,
    },
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Declaration {
    pub m_id: String,
    pub m_value: Option<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Assignment { m_name: String, m_value: Box<Expression> },
    Operation(ConditionalExpression),
}

#[derive(Debug)]
pub struct ConditionalExpression {
    pub m_condition: LogicalOrExpresson,
    pub m_true: Option<Box<Expression>>,
    pub m_false: Option<Box<ConditionalExpression>>,
}

#[derive(Debug)]
pub struct LogicalOrExpresson {
    // <logical-or-exp> ::= <logical-and-exp> { "||" <logical-and-exp> }
    pub m_first: Box<LogicalAndExpression>,
    pub m_rest: Vec<LogicalAndExpression>,
}

#[derive(Debug)]
pub struct LogicalAndExpression {
    // <logical-and-exp> ::= <equality-exp> { "&&" <equality-exp> }
    pub m_first: Box<EqualityExpression>,
    pub m_rest: Vec<EqualityExpression>,
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
    Modulo,
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
    FunCall { m_id: String, m_arguments: Vec<Expression> },
    Constant { m_value: i32 },
    UnaryOperation { m_opertator: UnaryOperator, m_factor: Box<Factor> },
    Braced { m_expression: Expression },
    Variable { m_var: String },
}

pub fn parse_program(tokens: &Vec<Token>) -> Result<Program, ParseError> {
    if DEBUG {
        println!("Paring program from: {:?}", &tokens);
    }

    let mut functions = Vec::new();

    let mut token_iter = tokens.iter().peekable();

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::EndOfFile => {
                token_iter.next();
                break;
            }
            _ => functions.push(match parse_function(&mut token_iter) {
                Ok(s) => s,
                Err(e) => return Err(e),
            }),
        }
    }

    return Ok(Program { m_functions: functions });
}

fn parse_function(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Function, ParseError> {
    if DEBUG {
        println!(
            "Parsing function from: {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    //Token Iterator

    let mut function = match token_iter.next() {
        Some(Token::KeywordInt) => Function {
            m_type: FunctionType::Int,
            m_params: Vec::new(),
            m_id: String::new(),
            m_items: Vec::new(),
        },
        Some(t) => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
        None => return Err(ParseError::ExpectedToken),
    };

    function.m_id = match token_iter.next() {
        Some(Token::Identifier(s)) => s.clone(),
        Some(t) => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
        None => return Err(ParseError::ExpectedToken),
    };

    match token_iter.next() {
        Some(Token::OpenParen) => (),
        Some(t) => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
        None => return Err(ParseError::ExpectedToken),
    }

    match token_iter.peek().cloned() {
        Some(Token::CloseParen) => (),
        Some(_) => {
            match token_iter.next() {
                Some(Token::KeywordInt) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseFunction,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
            let id = match token_iter.next() {
                Some(Token::Identifier(s)) => s,
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseFunction,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            };
            function.m_params.push(id.clone());
        }
        None => return Err(ParseError::ExpectedToken),
    }

    while let Some(next) = token_iter.peek().cloned() {
        match next {
            Token::Comma => {
                token_iter.next();

                match token_iter.next() {
                    Some(Token::KeywordInt) => (),
                    Some(t) => {
                        return Err(ParseError::UnexpectedToken(
                            t.clone(),
                            InFunction::ParseFunction,
                        ))
                    }
                    None => return Err(ParseError::ExpectedToken),
                }
                let id = match token_iter.next() {
                    Some(Token::Identifier(s)) => s,
                    Some(t) => {
                        return Err(ParseError::UnexpectedToken(
                            t.clone(),
                            InFunction::ParseFunction,
                        ))
                    }
                    None => return Err(ParseError::ExpectedToken),
                };
                function.m_params.push(id.clone());
            }
            Token::CloseParen => {
                token_iter.next();
                break;
            }
            t => {
                return Err(ParseError::UnexpectedToken(
                    t.clone(),
                    InFunction::ParseFunction,
                ))
            }
        }
    }

    let mut has_block: bool = false;

    match token_iter.next() {
        Some(Token::OpenBrace) => has_block = true,
        Some(Token::SemiColon) => (),
        Some(t) => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseFunction,
            ))
        }
        None => return Err(ParseError::ExpectedToken),
    }

    if has_block {
        while let Some(&next) = token_iter.peek() {
            match next {
                Token::CloseBrace => {
                    token_iter.next();
                    break;
                }
                _ => {
                    function.m_items.push(match parse_block_item(token_iter) {
                        Ok(s) => s,
                        Err(e) => return Err(e),
                    })
                }
            }
        }
    }

    return Ok(function);
}

fn parse_block_item(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<BlockItem, ParseError> {
    let block_item: BlockItem;
    if DEBUG {
        println!(
            "Parsing Block Item from: {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }

    match token_iter.peek().cloned() {
        Some(Token::KeywordInt) => {
            block_item =
                BlockItem::Declaration(match parse_declaration(token_iter) {
                    Ok(d) => d,
                    Err(e) => return Err(e),
                })
        }
        Some(_) => {
            block_item =
                BlockItem::Statement(match parse_statement(token_iter) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                })
        }
        None => return Err(ParseError::ExpectedToken),
    }

    if DEBUG {
        println!("Returning Block Item: {:?}", &block_item);
    }

    return Ok(block_item);
}

fn parse_declaration(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Declaration, ParseError> {
    let declaration: Declaration;

    if DEBUG {
        println!(
            "Parsing declaration from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }

    let token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    let id: String;
    let expression: Option<Expression>;

    match token {
        Token::KeywordInt => {
            let next_token = token_iter.next();
            match next_token {
                Some(Token::Identifier(s)) => id = s.clone(),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseDeclaration,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            };

            match token_iter.peek().cloned() {
                Some(Token::SemiColon) => expression = None,
                Some(Token::OperatorAssign) => {
                    token_iter.next();
                    expression = match parse_expression(token_iter) {
                        Ok(e) => Some(e),
                        Err(e) => return Err(e),
                    }
                }
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseDeclaration,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            match token_iter.next() {
                Some(Token::SemiColon) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseDeclaration,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            declaration = Declaration { m_id: id, m_value: expression };
        }
        t => {
            return Err(ParseError::UnexpectedToken(
                t.clone(),
                InFunction::ParseDeclaration,
            ))
        }
    }

    if DEBUG {
        println!("Returning Declaration: {:?}", &declaration);
    }

    return Ok(declaration);
}

fn parse_statement(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Statement, ParseError> {
    if DEBUG {
        println!(
            "Parsing statement from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    //Members
    let statement: Statement;
    //Token Iter

    match token_iter.peek().cloned() {
        Some(Token::KeywordFor) => {
            let mut initial_declaration: Option<Declaration> = None;
            let mut initial_exp: Option<Expression> = None;
            let condition: Expression;
            let mut post_expression: Option<Expression> = None;
            let loop_statement: Statement;

            token_iter.next();
            match token_iter.next() {
                Some(Token::OpenParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
            match token_iter.peek().cloned() {
                Some(Token::KeywordInt) => {
                    initial_declaration = match parse_declaration(token_iter) {
                        Ok(e) => Some(e),
                        Err(e) => return Err(e),
                    }
                }
                Some(Token::SemiColon) => {
                    token_iter.next();
                }
                Some(_) => {
                    initial_exp = match parse_expression(token_iter) {
                        Ok(e) => Some(e),
                        Err(e) => return Err(e),
                    };
                    match token_iter.next() {
                        Some(Token::SemiColon) => (),
                        Some(t) => {
                            return Err(ParseError::UnexpectedToken(
                                t.clone(),
                                InFunction::ParseStatement,
                            ))
                        }
                        None => return Err(ParseError::ExpectedToken),
                    }
                }
                None => return Err(ParseError::ExpectedToken),
            }

            // then condition

            match token_iter.peek().cloned() {
                Some(Token::SemiColon) => {
                    token_iter.next();
                    let new_tokens =
                        vec![Token::IntLiteral(1), Token::SemiColon];
                    let mut new_iter = new_tokens.iter().peekable();
                    condition = parse_expression(&mut new_iter).unwrap()
                }
                Some(_) => {
                    condition = match parse_expression(token_iter) {
                        Ok(e) => e,
                        Err(e) => return Err(e),
                    };
                    match token_iter.peek().cloned() {
                        Some(Token::SemiColon) => {
                            token_iter.next();
                        }
                        None => return Err(ParseError::ExpectedToken),
                        Some(t) => {
                            return Err(ParseError::UnexpectedToken(
                                t.clone(),
                                InFunction::ParseStatement,
                            ))
                        }
                    }
                }
                None => return Err(ParseError::ExpectedToken),
            }

            // then post-expression
            match token_iter.peek().cloned() {
                Some(Token::CloseParen) => (),
                Some(_) => {
                    post_expression = match parse_expression(token_iter) {
                        Ok(e) => Some(e),
                        Err(e) => return Err(e),
                    }
                }
                None => return Err(ParseError::ExpectedToken),
            }

            match token_iter.next() {
                Some(Token::CloseParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            loop_statement = match parse_statement(token_iter) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            match initial_exp {
                Some(e) => {
                    statement = Statement::For {
                        m_inititial_expression: Some(e),
                        m_condition: condition,
                        m_post_expression: post_expression,
                        m_statement: Box::new(loop_statement),
                    }
                }
                None => match initial_declaration {
                    Some(e) => {
                        statement = Statement::ForDecl {
                            m_initial_declaration: e,
                            m_condition: condition,
                            m_post_expression: post_expression,
                            m_statement: Box::new(loop_statement),
                        }
                    }
                    None => {
                        statement = Statement::For {
                            m_inititial_expression: initial_exp,
                            m_condition: condition,
                            m_post_expression: post_expression,
                            m_statement: Box::new(loop_statement),
                        }
                    }
                },
            }
        }

        Some(Token::KeywordWhile) => {
            let condition: Expression;
            let loop_statement: Statement;

            token_iter.next();
            match token_iter.next() {
                Some(Token::OpenParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            condition = match parse_expression(token_iter) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            match token_iter.next() {
                Some(Token::CloseParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            };

            loop_statement = match parse_statement(token_iter) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            statement = Statement::While {
                m_condition: condition,
                m_statement: Box::new(loop_statement),
            }
        }
        Some(Token::KeywordDo) => {
            let condition: Expression;
            let loop_statement: Statement;

            token_iter.next();

            loop_statement = match parse_statement(token_iter) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            match token_iter.next() {
                Some(Token::KeywordWhile) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            match token_iter.next() {
                Some(Token::OpenParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            condition = match parse_expression(token_iter) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            match token_iter.next() {
                Some(Token::CloseParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            match token_iter.next() {
                Some(Token::SemiColon) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            statement = Statement::Do {
                m_statement: Box::new(loop_statement),
                m_condition: condition,
            };
        }
        Some(Token::KeywordBreak) => {
            token_iter.next();
            statement = Statement::Break;
            match token_iter.next() {
                Some(Token::SemiColon) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
        }
        Some(Token::KeywordContinue) => {
            token_iter.next();
            statement = Statement::Continue;
            match token_iter.next() {
                Some(Token::SemiColon) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
        }
        Some(Token::OpenBrace) => {
            let mut block_items: Vec<Box<BlockItem>> = Vec::new();

            token_iter.next();

            while let Some(peeked) = token_iter.peek().cloned() {
                match peeked {
                    Token::CloseBrace => {
                        token_iter.next();
                        break;
                    }
                    _ => block_items.push(match parse_block_item(token_iter) {
                        Ok(s) => Box::new(s),
                        Err(e) => return Err(e),
                    }),
                }
            }

            statement = Statement::Compound { m_block_items: block_items };
        }
        Some(Token::KeywordIf) => {
            token_iter.next();

            match token_iter.next() {
                Some(Token::OpenParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            let condition = match parse_expression(token_iter) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            match token_iter.next() {
                Some(Token::CloseParen) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }

            let true_statement = match parse_statement(token_iter) {
                Ok(s) => Box::new(s),
                Err(e) => return Err(e),
            };

            let mut else_statement: Option<Box<Statement>> = None;

            match token_iter.peek().clone() {
                Some(Token::KeywordElse) => {
                    token_iter.next();
                    else_statement = Some(match parse_statement(token_iter) {
                        Ok(s) => Box::new(s),
                        Err(e) => return Err(e),
                    });
                }
                Some(_) => (),
                None => return Err(ParseError::ExpectedToken),
            }

            statement = Statement::If {
                m_condition: condition,
                m_true_statement: true_statement,
                m_else_statement: else_statement,
            };
        }
        Some(Token::KeywordReturn) => {
            token_iter.next();
            match token_iter.peek().cloned() {
                Some(Token::SemiColon) => {
                    return Err(ParseError::ExpectedToken)
                }
                Some(_) => {
                    let expression = match parse_expression(token_iter) {
                        Ok(e) => e,
                        Err(e) => return Err(e),
                    };
                    statement = Statement::Return(Some(expression));
                    match token_iter.next() {
                        Some(Token::SemiColon) => (),
                        Some(t) => {
                            return Err(ParseError::UnexpectedToken(
                                t.clone(),
                                InFunction::ParseStatement,
                            ))
                        }
                        None => return Err(ParseError::ExpectedToken),
                    }
                }
                None => return Err(ParseError::ExpectedToken),
            }
        }
        Some(Token::SemiColon) => {
            token_iter.next();
            statement = Statement::Expression(None);
        }
        Some(_) => {
            statement =
                Statement::Expression(match parse_expression(token_iter) {
                    Ok(exp) => Some(exp),
                    Err(e) => return Err(e),
                });
            match token_iter.next() {
                Some(Token::SemiColon) => (),
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseStatement,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
        }
        None => return Err(ParseError::ExpectedToken),
    }

    if DEBUG {
        println!("Returned statement: {:?}", &statement);
    }

    return Ok(statement);
}

fn parse_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Expression, ParseError> {
    if DEBUG {
        println!(
            "Parsing Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    let expression;

    match token_iter.peek().cloned() {
        Some(Token::Identifier(s)) => {
            let mut next_iter = token_iter.clone();
            next_iter.next();

            match next_iter.peek() {
                Some(Token::OperatorAssign) => {
                    token_iter.next();
                    token_iter.next();
                    let value = match parse_expression(token_iter) {
                        Ok(e) => e,
                        Err(e) => return Err(e),
                    };
                    expression = Expression::Assignment {
                        m_name: s.clone(),
                        m_value: Box::new(value),
                    };
                }
                Some(_) => {
                    expression = Expression::Operation(
                        match parse_conditional_expression(token_iter) {
                            Ok(l) => l,
                            Err(e) => return Err(e),
                        },
                    )
                }
                None => return Err(ParseError::ExpectedToken),
            };
        }

        Some(_) => {
            expression = Expression::Operation(
                match parse_conditional_expression(token_iter) {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                },
            )
        }

        None => return Err(ParseError::ExpectedToken),
    }

    if DEBUG {
        println!("Returning Expression: {:?}", &expression);
    }

    return Ok(expression);
}

fn parse_conditional_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<ConditionalExpression, ParseError> {
    if DEBUG {
        println!(
            "Parsing conditional expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }

    let conditional_expression;
    let exp = match parse_logical_or_expression(token_iter) {
        Ok(e) => e,
        Err(e) => return Err(e),
    };

    match token_iter.peek().cloned() {
        Some(Token::QuestionMark) => {
            token_iter.next();
            let true_exp = match parse_expression(token_iter) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };
            match token_iter.peek().cloned() {
                Some(Token::Colon) => {
                    token_iter.next();
                    let false_exp =
                        match parse_conditional_expression(token_iter) {
                            Ok(e) => e,
                            Err(e) => return Err(e),
                        };
                    conditional_expression = ConditionalExpression {
                        m_condition: exp,
                        m_true: Some(Box::new(true_exp)),
                        m_false: Some(Box::new(false_exp)),
                    }
                }
                Some(t) => {
                    return Err(ParseError::UnexpectedToken(
                        t.clone(),
                        InFunction::ParseConditionalExpression,
                    ))
                }
                None => return Err(ParseError::ExpectedToken),
            }
        }
        Some(_) => {
            conditional_expression = ConditionalExpression {
                m_condition: exp,
                m_true: None,
                m_false: None,
            }
        }
        None => return Err(ParseError::ExpectedToken),
    }

    if DEBUG {
        println!(
            "Returning conditional expression: {:?}",
            &conditional_expression
        );
    }

    return Ok(conditional_expression);
}

fn parse_logical_or_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<LogicalOrExpresson, ParseError> {
    if DEBUG {
        println!(
            "Parsing Logical Or Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    let mut logical_or_expression =
        match parse_logical_and_expression(token_iter) {
            Ok(l_a_e) => LogicalOrExpresson {
                m_first: Box::new(l_a_e),
                m_rest: Vec::new(),
            },
            Err(e) => return Err(e),
        };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorOr => {
                token_iter.next();
                logical_or_expression.m_rest.push(
                    match parse_logical_and_expression(token_iter) {
                        Ok(l_a_e) => l_a_e,
                        Err(e) => return Err(e),
                    },
                )
            }
            _ => break,
        }
    }

    if DEBUG {
        println!(
            "Returning Logical Or Expression: {:?}",
            &logical_or_expression
        );
    }

    return Ok(logical_or_expression);
}

fn parse_logical_and_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<LogicalAndExpression, ParseError> {
    if DEBUG {
        println!(
            "Parsing Logical And Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    let mut logical_and_expression = match parse_equality_expression(token_iter)
    {
        Ok(e_e) => {
            LogicalAndExpression { m_first: Box::new(e_e), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
        match next {
            Token::OperatorAnd => {
                token_iter.next();
                logical_and_expression.m_rest.push(
                    match parse_equality_expression(token_iter) {
                        Ok(e_e) => e_e,
                        Err(e) => return Err(e),
                    },
                )
            }
            _ => break,
        }
    }

    if DEBUG {
        println!(
            "Returning logical and expression: {:?}",
            &logical_and_expression
        );
    }

    return Ok(logical_and_expression);
}

fn parse_equality_expression(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<EqualityExpression, ParseError> {
    if DEBUG {
        println!(
            "Parsing Equality Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
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
    if DEBUG {
        println!(
            "Parsing Relational Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
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
    if DEBUG {
        println!(
            "Parsing Additive Expression from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    //Members
    let mut additive_expression = match parse_term(token_iter) {
        Ok(t) => {
            AdditiveExpression { m_first_term: Box::new(t), m_rest: Vec::new() }
        }
        Err(e) => return Err(e),
    };

    while let Some(&next) = token_iter.peek() {
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
    if DEBUG {
        println!(
            "Parsing term from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
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
            Token::OperatorModulo => {
                token_iter.next();
                term.m_rest.push((
                    MultiplicativeOperator::Modulo,
                    match parse_factor(token_iter) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    },
                ))
            }
            _ => break,
        }
    }

    if DEBUG {
        println!("returning term {:?}", term);
    }

    return Ok(term);
}

fn parse_factor(
    token_iter: &mut std::iter::Peekable<Iter<Token>>,
) -> Result<Factor, ParseError> {
    if DEBUG {
        println!(
            "Parsing factor from {:?}",
            token_iter.clone().take(5).collect::<Vec<_>>()
        );
    }
    let factor: Factor;
    let cur_token = match token_iter.next() {
        Some(t) => t,
        None => return Err(ParseError::ExpectedToken),
    };

    match cur_token {
        Token::Identifier(s) => {
            match token_iter.peek().cloned() {
                Some(Token::OpenParen) => {
                    let mut arguments: Vec<Expression> = Vec::new();
                    token_iter.next();
                    match token_iter.peek().cloned() {
                        Some(Token::CloseParen) => (),
                        Some(_) => {
                            arguments.push(
                                match parse_expression(token_iter) {
                                    Ok(e) => e,
                                    Err(e) => return Err(e),
                                },
                            );
                        }
                        None => return Err(ParseError::ExpectedToken),
                    }
                    while let Some(&next) = token_iter.peek() {
                        match next {
                            Token::CloseParen => {
                                token_iter.next();
                                break;
                            }
                            Token::Comma => {
                                token_iter.next();
                                arguments.push(
                                    match parse_expression(token_iter) {
                                        Ok(e) => e,
                                        Err(e) => return Err(e),
                                    },
                                );
                            }
                            t => {
                                return Err(ParseError::UnexpectedToken(
                                    t.clone(),
                                    InFunction::ParseFactor,
                                ))
                            }
                        }
                    }

                    factor = Factor::FunCall {
                        m_id: s.clone(),
                        m_arguments: arguments,
                    };
                }
                Some(_) => factor = Factor::Variable { m_var: s.clone() },
                None => return Err(ParseError::ExpectedToken),
            };
        }
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

    if DEBUG {
        println!("returning factor {:?}", factor);
    }

    return Ok(factor);
}
