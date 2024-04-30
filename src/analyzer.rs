use crate::parser::{
    AdditiveExpression, BlockItem, ConditionalExpression, Declaration,
    EqualityExpression, Expression, Factor, Function, FunctionType,
    LogicalAndExpression, LogicalOrExpresson, Program, RelationalExpression,
    Statement, Term, VarType,
};

#[derive(Debug)]
struct FunctionDef {
    pub m_id: String,
    pub m_type: FunctionType,
    pub m_parameters: Vec<(VarType, String)>,
}

impl FunctionDef {
    pub fn clone(self) -> Self {
        return FunctionDef {
            m_id: self.m_id.clone(),
            m_type: self.m_type.clone(),
            m_parameters: self.m_parameters.clone(),
        };
    }
}

const DEBUG: bool = false;

pub struct Analyzer {
    m_functions: Vec<FunctionDef>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer { m_functions: Vec::new() }
    }

    fn num_arguments(&self, id: &String) -> Option<usize> {
        for func in &self.m_functions {
            if &func.m_id == id {
                return Some(func.m_parameters.len());
            }
        }
        return None;
    }

    pub fn analyze_program(&mut self, program: &Program) -> bool {
        for func in &program.m_functions {
            match self.analyze_function(&func) {
                true => (),
                false => return false,
            }
        }
        return true;
    }

    pub fn analyze_function(&mut self, function: &Function) -> bool {
        if DEBUG {
            println!("Analyzing Function: {:?}", &function);
        }
        let valid = true;
        match self.num_arguments(&function.m_id) {
            Some(n) => {
                if function.m_params.len() != n {
                    return false;
                }
            }
            None => {
                let new_function = FunctionDef {
                    m_id: function.m_id.clone(),
                    m_type: function.m_type.clone(),
                    m_parameters: function.m_params.clone(),
                };
                self.m_functions.push(new_function);
            }
        }

        match &function.m_items {
            Some(b) => {
                for item in b {
                    if !self.analyze_block(&item) {
                        return false;
                    }
                }
            }
            None => (),
        }

        return valid;
    }

    fn analyze_block(&mut self, item: &BlockItem) -> bool {
        if DEBUG {
            println!("Analyzing BlockItem: {:?}", &item);
        }
        match item {
            BlockItem::Statement(statement) => {
                return self.analyze_statement(statement)
            }
            BlockItem::Declaration(declaration) => {
                return self.analyze_declaration(declaration)
            }
        }
    }

    fn analyze_declaration(&mut self, declaration: &Declaration) -> bool {
        if DEBUG {
            println!("Analyzing Declaration: {:?}", &declaration);
        }
        match &declaration.m_value {
            Some(e) => return self.analyze_expression(e),
            None => return true,
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) -> bool {
        if DEBUG {
            println!("Analyzing Statement: {:?}", &statement);
        }
        match statement {
            Statement::Return(e) => match e {
                None => return true,
                Some(exp) => return self.analyze_expression(exp),
            },
            Statement::Expression(e) => match e {
                None => return true,
                Some(exp) => return self.analyze_expression(exp),
            },
            Statement::If {
                m_condition,
                m_true_statement,
                m_else_statement,
            } => {
                match m_else_statement {
                    Some(s) => {
                        if !self.analyze_statement(s) {
                            return false;
                        }
                    }
                    None => (),
                }
                return self.analyze_expression(m_condition)
                    && self.analyze_statement(m_true_statement);
            }
            Statement::Compound { m_block_items } => {
                for block_item in m_block_items {
                    if !self.analyze_block(block_item) {
                        return false;
                    }
                }
                return true;
            }
            Statement::For {
                m_initial_expression,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                match m_initial_expression {
                    None => (),
                    Some(e) => {
                        if !self.analyze_expression(e) {
                            return false;
                        }
                    }
                }
                match m_post_expression {
                    None => (),
                    Some(e) => {
                        if !self.analyze_expression(e) {
                            return false;
                        }
                    }
                }
                return self.analyze_expression(m_condition)
                    && self.analyze_statement(m_statement);
            }
            Statement::ForDecl {
                m_initial_declaration,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                match m_post_expression {
                    None => (),
                    Some(e) => {
                        if !self.analyze_expression(e) {
                            return false;
                        }
                    }
                }
                return self.analyze_expression(m_condition)
                    && self.analyze_declaration(m_initial_declaration)
                    && self.analyze_statement(m_statement);
            }
            Statement::While { m_condition, m_statement } => {
                return self.analyze_expression(m_condition)
                    && self.analyze_statement(m_statement);
            }
            Statement::Do { m_statement, m_condition } => {
                return self.analyze_expression(m_condition)
                    && self.analyze_statement(m_statement);
            }
            Statement::Break => return true,
            Statement::Continue => return true,
        }
    }

    fn analyze_expression(&mut self, expression: &Expression) -> bool {
        if DEBUG {
            println!("Analyzing Expression: {:?}", &expression);
        }
        match expression {
            Expression::Assignment { m_name, m_value } => {
                return self.analyze_expression(m_value);
            }
            Expression::Operation(conditional_expression) => {
                return self
                    .analyze_conditional_expression(conditional_expression)
            }
        }
    }

    fn analyze_conditional_expression(
        &mut self,
        conditional_expression: &ConditionalExpression,
    ) -> bool {
        if DEBUG {
            println!(
                "Analyzing ConditionalExpression: {:?}",
                &conditional_expression
            );
        }
        match &conditional_expression.m_true {
            Some(e) => {
                if !self.analyze_expression(&e) {
                    return false;
                }
            }
            None => (),
        }
        match &conditional_expression.m_false {
            Some(e) => {
                if !self.analyze_conditional_expression(&e) {
                    return false;
                }
            }
            None => (),
        }
        return self.analyze_logical_or_expression(
            &conditional_expression.m_condition,
        );
    }

    fn analyze_logical_or_expression(
        &mut self,
        logical_or_expression: &LogicalOrExpresson,
    ) -> bool {
        if DEBUG {
            println!(
                "Analyzing LogicalOrExpression: {:?}",
                &logical_or_expression
            );
        }
        if !(self
            .analyze_logical_and_expression(&logical_or_expression.m_first))
        {
            return false;
        }
        for next in &logical_or_expression.m_rest {
            if !(self.analyze_logical_and_expression(&next)) {
                return false;
            }
        }
        return true;
    }

    fn analyze_logical_and_expression(
        &mut self,
        logical_and_expression: &LogicalAndExpression,
    ) -> bool {
        if DEBUG {
            println!(
                "Analyzing LogicalAndExpression: {:?}",
                &logical_and_expression
            );
        }
        if !(self.analyze_equality_expression(&logical_and_expression.m_first))
        {
            return false;
        }
        for next in &logical_and_expression.m_rest {
            if !(self.analyze_equality_expression(&next)) {
                return false;
            }
        }
        return true;
    }

    fn analyze_equality_expression(
        &mut self,
        equality_expession: &EqualityExpression,
    ) -> bool {
        if !self.analyze_relational_expression(&equality_expession.m_first) {
            return false;
        }
        for next in &equality_expession.m_rest {
            if !self.analyze_relational_expression(&next.1) {
                return false;
            }
        }
        return true;
    }

    fn analyze_relational_expression(
        &mut self,
        relational_expession: &RelationalExpression,
    ) -> bool {
        if DEBUG {
            println!(
                "Analyzing RelationalExpression: {:?}",
                &relational_expession
            );
        }
        if !self.analyze_additive_expression(&relational_expession.m_first) {
            return false;
        }
        for next in &relational_expession.m_rest {
            if !self.analyze_additive_expression(&next.1) {
                return false;
            }
        }
        return true;
    }

    fn analyze_additive_expression(
        &mut self,
        additive_expression: &AdditiveExpression,
    ) -> bool {
        if DEBUG {
            println!(
                "Analyzing AdditiveExpression: {:?}",
                &additive_expression
            );
        }
        if !self.analyze_term(&additive_expression.m_first_term) {
            return false;
        }
        for next in &additive_expression.m_rest {
            if !self.analyze_term(&next.1) {
                return false;
            }
        }
        return true;
    }

    fn analyze_term(&mut self, term: &Term) -> bool {
        if DEBUG {
            println!("Analyzing Term: {:?}", &term);
        }
        if !self.analyze_factor(&term.m_first_factor) {
            return false;
        }
        for next in &term.m_rest {
            if !self.analyze_factor(&next.1) {
                return false;
            }
        }
        return true;
    }

    fn analyze_factor(&mut self, factor: &Factor) -> bool {
        if DEBUG {
            println!("Analyzing Factor: {:?}", &factor);
        }
        match factor {
            Factor::FunCall { m_id, m_arguments } => {
                match self.num_arguments(&m_id) {
                    None => {
                        return false;
                    }
                    Some(n) => {
                        if m_arguments.len() != n {
                            return false;
                        } else {
                            return true;
                        }
                    }
                }
            }
            Factor::Constant { m_value } => return true,
            Factor::UnaryOperation { m_opertator, m_factor } => {
                return self.analyze_factor(&m_factor)
            }
            Factor::Braced { m_expression } => {
                return self.analyze_expression(&m_expression)
            }
            Factor::Variable { m_var } => return true,
        }
    }
}
