use crate::parser::{
    AdditiveExpression, BlockItem, ConditionalExpression, Declaration,
    EqualityExpression, Expression, Factor, Function, FunctionType,
    LogicalAndExpression, LogicalOrExpresson, Program, RelationalExpression,
    Statement, Term, VarType,
};

use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum AnalysisError {
    TypeError(String, String),
    UndeclaredIdentifier(String, String),
    UninitializedVariable(String, String),
    ReturnError(String, String),
    AssignmentError(String, String),
    DuplicateDeclaration(String, String),
    FunctionError(String, String),
}

#[derive(Debug)]
struct FunctionDef {
    pub m_type: FunctionType,
    pub m_parameters: Vec<(VarType, String)>,
}

impl FunctionDef {
    pub fn clone(self) -> Self {
        return FunctionDef {
            m_type: self.m_type.clone(),
            m_parameters: self.m_parameters.clone(),
        };
    }
}

#[derive(Debug)]
enum Symbol {
    Func(FunctionDef),
    Var(VarType),
}

const DEBUG: bool = false;

pub struct Analyser {
    context: Vec<HashMap<String, Symbol>>,
}

impl Analyser {
    pub fn new() -> Self {
        Analyser { context: Vec::new() }
    }

    fn open_scope(&mut self) {
        self.context.push(HashMap::new());
    }

    fn close_scope(&mut self) {
        self.context.pop();
    }

    fn add_function(&mut self, id: String, func: FunctionDef) {
        self.context.last_mut().unwrap().insert(id, Symbol::Func(func));
    }

    fn add_var(&mut self, id: String, var_type: VarType) {
        self.context.last_mut().unwrap().insert(id, Symbol::Var(var_type));
    }

    fn num_arguments(&self, id: &String) -> Option<usize> {
        for context in self.context.iter().rev() {
            match context.get(id) {
                Some(sym) => match sym {
                    Symbol::Func(f_def) => {
                        return Some(f_def.m_parameters.len());
                    }
                    Symbol::Var(_) => return None,
                },
                None => (),
            }
        }
        return None;
    }

    pub fn analyse_program(
        &mut self,
        program: &Program,
    ) -> Result<bool, AnalysisError> {
        self.open_scope();
        for func in &program.m_functions {
            match self.analyse_function(&func) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    pub fn analyse_function(
        &mut self,
        function: &Function,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Function: {:?}", &function);
        }
        match self.num_arguments(&function.m_id) {
            Some(n) => {
                if function.m_params.len() != n {
                    return Err(AnalysisError::FunctionError(
                        function.m_id.clone(),
                        String::from("Function declaration type error"),
                    ));
                }
            }
            None => {
                let new_function = FunctionDef {
                    m_type: function.m_type.clone(),
                    m_parameters: function.m_params.clone(),
                };
                self.add_function(function.m_id.clone(), new_function);
            }
        }

        self.open_scope();

        match &function.m_items {
            Some(b) => {
                for item in b {
                    match self.analyse_block_item(&item) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
            None => (),
        }

        self.close_scope();

        return Ok(true);
    }

    fn analyse_block_item(
        &mut self,
        item: &BlockItem,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing BlockItem: {:?}", &item);
        }
        match item {
            BlockItem::Statement(statement) => {
                return self.analyse_statement(&statement)
            }
            BlockItem::Declaration(declaration) => {
                return self.analyse_declaration(&declaration)
            }
        }
    }

    fn analyse_declaration(
        &mut self,
        declaration: &Declaration,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Declaration: {:?}", &declaration);
        }
        match &declaration.m_value {
            Some(e) => return self.analyse_expression(e),
            None => return Ok(true),
        }
    }

    fn analyse_statement(
        &mut self,
        statement: &Statement,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Statement: {:?}", &statement);
        }
        match statement {
            Statement::Return(e) => match e {
                None => return Ok(true),
                Some(exp) => return self.analyse_expression(exp),
            },
            Statement::Expression(e) => match e {
                None => return Ok(true),
                Some(exp) => return self.analyse_expression(exp),
            },
            Statement::If {
                m_condition,
                m_true_statement,
                m_else_statement,
            } => {
                match m_else_statement {
                    Some(s) => {
                        match self.analyse_statement(s) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        };
                    }
                    None => (),
                }
                match self.analyse_expression(m_condition) {
                    Ok(_) => {
                        return self.analyse_statement(&m_true_statement);
                    }
                    Err(e) => return Err(e),
                }
            }
            Statement::Compound { m_block_items } => {
                self.open_scope();
                for block_item in m_block_items {
                    match self.analyse_block_item(block_item) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
                self.close_scope();
                return Ok(true);
            }
            Statement::For {
                m_initial_expression,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                self.open_scope();
                match m_initial_expression {
                    None => (),
                    Some(e) => {
                        match self.analyse_expression(&e) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        };
                    }
                }
                match m_post_expression {
                    None => (),
                    Some(e) => match self.analyse_expression(&e) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    },
                }
                match self.analyse_expression(&m_condition) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match self.analyse_statement(&m_statement) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                self.close_scope();
                return Ok(true);
            }
            Statement::ForDecl {
                m_initial_declaration,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                self.open_scope();
                match m_post_expression {
                    None => (),
                    Some(e) => match self.analyse_expression(&e) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    },
                }
                match self.analyse_expression(&m_condition) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match self.analyse_declaration(&m_initial_declaration) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match self.analyse_statement(&m_statement) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                self.close_scope();
                return Ok(true);
            }
            Statement::While { m_condition, m_statement } => {
                match self.analyse_expression(&m_condition) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match self.analyse_statement(m_statement) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                return Ok(true);
            }
            Statement::Do { m_statement, m_condition } => {
                match self.analyse_expression(&m_condition) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match self.analyse_statement(m_statement) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                return Ok(true);
            }
            Statement::Break => return Ok(true),
            Statement::Continue => return Ok(true),
        }
    }

    fn analyse_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Expression: {:?}", &expression);
        }
        match expression {
            Expression::Assignment { m_name, m_value } => {
                return self.analyse_expression(m_value);
            }
            Expression::Operation(conditional_expression) => {
                return self
                    .analyse_conditional_expression(conditional_expression)
            }
        }
    }

    fn analyse_conditional_expression(
        &mut self,
        conditional_expression: &ConditionalExpression,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!(
                "Analyzing ConditionalExpression: {:?}",
                &conditional_expression
            );
        }
        match &conditional_expression.m_true {
            Some(e) => match self.analyse_expression(&e) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }
        match &conditional_expression.m_false {
            Some(e) => match self.analyse_conditional_expression(&e) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }
        return self.analyse_logical_or_expression(
            &conditional_expression.m_condition,
        );
    }

    fn analyse_logical_or_expression(
        &mut self,
        logical_or_expression: &LogicalOrExpresson,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!(
                "Analyzing LogicalOrExpression: {:?}",
                &logical_or_expression
            );
        }
        match self
            .analyse_logical_and_expression(&logical_or_expression.m_first)
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &logical_or_expression.m_rest {
            match self.analyse_logical_and_expression(&next) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_logical_and_expression(
        &mut self,
        logical_and_expression: &LogicalAndExpression,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!(
                "Analyzing LogicalAndExpression: {:?}",
                &logical_and_expression
            );
        }
        match self.analyse_equality_expression(&logical_and_expression.m_first)
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &logical_and_expression.m_rest {
            match self.analyse_equality_expression(&next) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_equality_expression(
        &mut self,
        equality_expession: &EqualityExpression,
    ) -> Result<bool, AnalysisError> {
        match self.analyse_relational_expression(&equality_expession.m_first) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &equality_expession.m_rest {
            match self.analyse_relational_expression(&next.1) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_relational_expression(
        &mut self,
        relational_expession: &RelationalExpression,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!(
                "Analyzing RelationalExpression: {:?}",
                &relational_expession
            );
        }
        match self.analyse_additive_expression(&relational_expession.m_first) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &relational_expession.m_rest {
            match self.analyse_additive_expression(&next.1) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_additive_expression(
        &mut self,
        additive_expression: &AdditiveExpression,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!(
                "Analyzing AdditiveExpression: {:?}",
                &additive_expression
            );
        }
        match self.analyse_term(&additive_expression.m_first_term) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &additive_expression.m_rest {
            match self.analyse_term(&next.1) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_term(&mut self, term: &Term) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Term: {:?}", &term);
        }
        match self.analyse_factor(&term.m_first_factor) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for next in &term.m_rest {
            match self.analyse_factor(&next.1) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(true);
    }

    fn analyse_factor(
        &mut self,
        factor: &Factor,
    ) -> Result<bool, AnalysisError> {
        if DEBUG {
            println!("Analyzing Factor: {:?}", &factor);
        }
        match factor {
            Factor::FunCall { m_id, m_arguments } => {
                match self.num_arguments(&m_id) {
                    None => {
                        return Err(AnalysisError::FunctionError(
                            m_id.clone(),
                            String::from("Undefined identifier"),
                        ));
                    }
                    Some(n) => {
                        if m_arguments.len() != n {
                            return Err(AnalysisError::FunctionError(
                                m_id.clone(),
                                String::from("Invalid Arguments"),
                            ));
                        } else {
                            return Ok(true);
                        }
                    }
                }
            }
            Factor::Constant { m_value } => return Ok(true),
            Factor::UnaryOperation { m_opertator, m_factor } => {
                return self.analyse_factor(&m_factor)
            }
            Factor::Braced { m_expression } => {
                return self.analyse_expression(&m_expression)
            }
            Factor::Variable { m_var } => return Ok(true),
        }
    }
}
