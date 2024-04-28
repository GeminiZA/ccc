use std::os::linux::raw::stat;
use std::panic::Location;
use std::{collections::HashMap, task::Context};

use crate::parser::{
    AdditiveExpression, AdditiveOperator, BlockItem, ConditionalExpression,
    Declaration, EqualityExpression, EqualityOperator, Expression, Factor,
    Function, LogicalAndExpression, LogicalOrExpresson, MultiplicativeOperator,
    Program, RelationalExpression, RelationalOperator, Statement, Term,
    UnaryOperator,
};

static mut LABEL_NUMBER: i32 = 0;

pub struct LoopContext {
    start_label: String,
    end_label: String,
    scope_count: usize,
}

pub struct Generator {
    label_number: i32,
    context: Vec<HashMap<String, i32>>,
    stack_index: i32,
    returned: bool,
    loop_contexts: Vec<LoopContext>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            label_number: 0,
            context: Vec::new(),
            stack_index: -8,
            returned: false,
            loop_contexts: Vec::new(),
        }
    }

    fn enter_loop(&mut self, start_label: String, end_label: String) {
        let scope_count = self.context.len();
        self.loop_contexts.push(LoopContext {
            start_label,
            end_label,
            scope_count,
        });
    }

    fn last_end_label(self) -> String {
        if let Some(&loop_context) = self.loop_contexts.last() {
            return loop_context.start_label.clone();
        } else {
            panic!("Got label not in a loop context")
        }
    }

    fn last_start_label(self) -> String {
        if let Some(&loop_context) = self.loop_contexts.last() {
            return loop_context.start_label.clone();
        } else {
            panic!("Got label not in a loop context")
        }
    }

    fn leave_loop(&mut self) {
        self.loop_contexts.pop();
    }

    fn open_scope(&mut self) {
        self.context.push(HashMap::new());
    }

    fn add_var(&mut self, var_name: &String) {
        let mut current_context = self.context.last_mut().unwrap();
        match current_context.get(var_name) {
            Some(v) => {
                panic!("Variable {} already declared in this scope!", &var_name)
            }
            None => (),
        }
        current_context.insert(var_name.clone(), self.stack_index);
        self.stack_index = self.stack_index - 8;
    }

    fn query_var(&mut self, var_name: &String) -> Option<i32> {
        for scope in self.context.iter().rev() {
            if let Some(&value) = scope.get(var_name) {
                return Some(value);
            }
        }
        return None;
    }

    fn close_scope(&mut self) -> usize {
        let cur_context = match self.context.pop() {
            Some(c) => c,
            None => return 0,
        };
        let size = cur_context.len();
        self.stack_index = self.stack_index + 8 * size as i32;
        return size * 8;
    }

    fn generate_label(&mut self) -> String {
        let label = format!("label_{}", self.label_number);
        self.label_number += 1;
        return label;
    }

    pub fn generate(&mut self, program: &Program) -> String {
        let mut gen_s: String = String::new();

        gen_s.push_str(&self.generate_function(&program.m_function));

        return gen_s;
    }

    fn generate_function(&mut self, function: &Function) -> String {
        let mut gen_s: String = String::new();

        self.open_scope();

        gen_s.push_str(
            format!(
                ".globl {0}\n\
                {0}:\n\
                \tpushq\t%rbp\n\
                \tmovq\t%rsp, %rbp\n",
                function.m_id
            )
            .as_str(),
        );

        for block_item in &function.m_items {
            gen_s.push_str(&self.generate_block_item(&block_item));
        }

        if !self.returned {
            gen_s.push_str(
                format!(
                    "\tmovq\t%rbp, %rsp\n\
                    \tpop \t%rbp\n\
                    \tmovq\t$0, %rax\n\
                    \tret\n"
                )
                .as_str(),
            );
        }

        self.close_scope();

        return gen_s;
    }

    fn generate_block_item(&mut self, block_item: &BlockItem) -> String {
        let mut gen_s = String::new();
        match &block_item {
            BlockItem::Declaration(declaration) => {
                gen_s.push_str(&self.generate_declaration(&declaration))
            }
            BlockItem::Statement(statement) => {
                gen_s.push_str(&self.generate_statement(&statement))
            }
        }

        return gen_s;
    }

    fn generate_statement(&mut self, statement: &Statement) -> String {
        let mut gen_s: String = String::new();

        match statement {
            Statement::Continue => {
                if let Some(loop_context) = self.loop_contexts.last() {
                    let mut size_to_deallocate: usize = 0;
                    while self.context.len() > loop_context.scope_count {
                        size_to_deallocate += self.close_scope();
                    }
                    gen_s.push_str(
                        format!("\taddq\t${}, %rsp\n", size_to_deallocate * 8)
                            .as_str(),
                    );
                } else {
                    panic!("continue statement not in loop context");
                }
            }
            Statement::Break => {
                if let Some(loop_context) = self.loop_contexts.last() {
                    let mut size_to_deallocate: usize = 0;
                    while self.context.len() > loop_context.scope_count {
                        size_to_deallocate += self.close_scope();
                    }
                    gen_s.push_str(
                        format!("\taddq\t${}, %rsp\n", size_to_deallocate * 8)
                            .as_str(),
                    );
                } else {
                    panic!("break statement not in loop context");
                }
            }
            Statement::ForDecl {
                m_initial_declaration,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                self.enter_loop(self.generate_label(), self.generate_label());
                self.open_scope();
                let condition_label = self.generate_label();
                gen_s.push_str(
                    &self.generate_declaration(&m_initial_declaration),
                );
                gen_s.push_str(format!("{}:\n", &condition_label).as_str());
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                    je\t{0}\n",
                        &self.last_end_label()
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_statement(&m_statement));
                gen_s.push_str(
                    format!("{}:\n", &self.last_start_label()).as_str(),
                );
                match m_post_expression {
                    Some(e) => gen_s.push_str(&self.generate_expression(&e)),
                    None => (),
                }
                gen_s.push_str(
                    format!("\tjmp\t{}\n", &condition_label).as_str(),
                );
                gen_s.push_str(
                    format!("{}:\n", &self.last_end_label()).as_str(),
                );
                self.leave_loop();
                let size_to_deallocate = self.close_scope();
                gen_s.push_str(
                    format!("\taddq\t${}, %rsp\n", size_to_deallocate).as_str(),
                );
            }

            Statement::For {
                m_inititial_expression,
                m_condition,
                m_post_expression,
                m_statement,
            } => {
                self.enter_loop(self.generate_label(), self.generate_label());
                self.open_scope();
                let condition_label = self.generate_label();
                match m_inititial_expression {
                    Some(e) => gen_s.push_str(&self.generate_expression(&e)),
                    None => (),
                }
                gen_s.push_str(format!("{}:\n", &condition_label).as_str());
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                    je\t{0}\n",
                        &self.last_end_label()
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_statement(&m_statement));
                gen_s.push_str(
                    format!("{}:\n", &self.last_start_label()).as_str(),
                );
                match m_post_expression {
                    Some(e) => gen_s.push_str(&self.generate_expression(&e)),
                    None => (),
                }
                gen_s.push_str(
                    format!("\tjmp\t{}\n", &condition_label).as_str(),
                );
                gen_s.push_str(
                    format!("{}:\n", &self.last_end_label()).as_str(),
                );
                self.leave_loop();
                let size_to_deallocate = self.close_scope();
                gen_s.push_str(
                    format!("\taddq\t${}, %rsp\n", size_to_deallocate).as_str(),
                );
            }
            Statement::While { m_condition, m_statement } => {
                self.enter_loop(self.generate_label(), self.generate_label());
                self.open_scope();
                gen_s.push_str(
                    format!("{}:\n", &self.last_start_label()).as_str(),
                );
                gen_s.push_str(&self.generate_expression(&m_condition));
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                    je\t{0}\n",
                        &self.last_end_label()
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_statement(&m_statement));
                gen_s.push_str(
                    format!("\tjmp\t{}\n", &self.last_start_label()).as_str(),
                );
                gen_s.push_str(
                    format!("{}:\n", &self.last_end_label()).as_str(),
                );

                self.leave_loop();
                let size_to_deallocate = self.close_scope();

                gen_s.push_str(
                    format!("\taddq\t${}, %rsp\n", size_to_deallocate).as_str(),
                );
            }
            Statement::Do { m_statement, m_condition } => {
                self.enter_loop(self.generate_label(), self.generate_label());
                self.open_scope();
                let start_label = self.last_start_label();
                let end_label = self.last_end_label();
                gen_s.push_str(format!("{}:\n", &start_label).as_str());
                gen_s.push_str(&self.generate_statement(&m_statement));
                gen_s.push_str(&self.generate_expression(&m_condition));
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                    je\t{0}\n",
                        &self.last_end_label()
                    )
                    .as_str(),
                );
                gen_s.push_str(
                    format!("\tjmp\t{}\n", &self.last_start_label()).as_str(),
                );
                gen_s.push_str(
                    format!("{}:\n", &self.last_end_label()).as_str(),
                );

                self.leave_loop();

                let size_to_deallocate = self.close_scope();

                gen_s.push_str(
                    format!("\taddq\t${}, %rsp\n", size_to_deallocate).as_str(),
                );
            }
            Statement::Compound { m_block_items } => {
                self.open_scope();

                for block_item in m_block_items {
                    gen_s.push_str(&self.generate_block_item(block_item));
                }

                let size_to_deallocate = self.close_scope();

                gen_s.push_str(
                    format!("\taddq\t${}, %rsp\n", size_to_deallocate).as_str(),
                );
            }
            Statement::Expression(expression) => match expression {
                Some(e) => gen_s.push_str(&self.generate_expression(&e)),
                None => (),
            },

            Statement::Return(expression) => {
                self.returned = true;
                match expression {
                    Some(e) => gen_s.push_str(&self.generate_expression(&e)),
                    None => (),
                }
                gen_s.push_str(
                    format!(
                        "\tmovq\t%rbp, %rsp\n\
                        \tpop \t%rbp\n\
                        \tret\n"
                    )
                    .as_str(),
                );
            }

            Statement::If {
                m_condition,
                m_true_statement,
                m_else_statement,
            } => {
                let false_label = &self.generate_label();
                let end_label = &self.generate_label();
                gen_s.push_str(&self.generate_expression(&m_condition));
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                    je\t{0}\n",
                        &false_label
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_statement(&m_true_statement));
                gen_s.push_str(
                    format!(
                        "\tjmp\t\t{0}\n\
                    {1}:\n",
                        &end_label, &false_label,
                    )
                    .as_str(),
                );
                match m_else_statement {
                    Some(s) => gen_s.push_str(&self.generate_statement(&s)),
                    None => (),
                }
                gen_s.push_str(format!("{}:", &end_label).as_str());
            }
        }

        return gen_s;
    }

    fn generate_declaration(&mut self, declaration: &Declaration) -> String {
        let mut gen_s = String::new();

        match &declaration.m_value {
            Some(e) => {
                gen_s.push_str(&self.generate_expression(&e));
                gen_s.push_str("\tpushq\t%rax\n");
            }
            None => (),
        }
        self.add_var(&declaration.m_id);

        return gen_s;
    }

    fn generate_expression(&mut self, expression: &Expression) -> String {
        let mut gen_s = String::new();
        match expression {
            Expression::Assignment { m_name, m_value } => {
                gen_s.push_str(&self.generate_expression(&m_value));
                let var_offset = match self.query_var(&m_name) {
                    Some(i) => i,
                    None => panic!("Use of undeclared variable {}", m_name),
                };
                gen_s.push_str(
                    format!("\tmovq\t%rax, {}(%rbp)\n", var_offset).as_str(),
                );
            }
            Expression::Operation(conditional_expression) => gen_s.push_str(
                &self.generate_conditional_expression(&conditional_expression),
            ),
        }

        return gen_s;
    }

    fn generate_conditional_expression(
        &mut self,
        conditional_expression: &ConditionalExpression,
    ) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(&self.generate_logical_or_expression(
            &conditional_expression.m_condition,
        ));

        match &conditional_expression.m_true {
            Some(exp) => {
                let false_lable = &self.generate_label();
                let end_label = &self.generate_label();
                gen_s.push_str(
                    format!(
                        "\tcmpq\t$0, %rax\n\
                        \tje\t\t{}\n",
                        &false_lable
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_expression(&exp));
                gen_s.push_str(
                    format!(
                        "\tjmp\t\t{0}\n\
                        {1}:\n",
                        &end_label, &false_lable
                    )
                    .as_str(),
                );
                gen_s.push_str(&self.generate_conditional_expression(
                    &conditional_expression.m_false.as_ref().unwrap(),
                ));
                gen_s.push_str(format!("{}:\n", &end_label).as_str());
            }
            None => (),
        }

        return gen_s;
    }

    fn generate_logical_or_expression(
        &mut self,
        logical_or_expression: &LogicalOrExpresson,
    ) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(
            &self.generate_logical_and_expression(
                &logical_or_expression.m_first,
            ),
        );

        for next_op in &logical_or_expression.m_rest {
            let e2_label = &self.generate_label();
            let end_label = &self.generate_label();
            gen_s.push_str(
                format!(
                    "\tcmpq\t$0, %rax\n\
                \tje\t{0}\n\
                \tmovq\t$1, %rax\n\
                \tjmp\t{1}\n\
                {0}:\n",
                    &e2_label, &end_label,
                )
                .as_str(),
            );
            gen_s.push_str(&self.generate_logical_and_expression(&next_op));
            gen_s.push_str(
                format!(
                    "\tcmpq\t$0, %rax\n\
                \tmovq\t$0, %rax\n\
                \tsetne\t%al\n\
                {0}:",
                    &end_label
                )
                .as_str(),
            );
        }

        return gen_s;
    }

    fn generate_logical_and_expression(
        &mut self,
        logical_and_expresson: &LogicalAndExpression,
    ) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(
            &self.generate_equality_expression(&logical_and_expresson.m_first),
        );

        for next_op in &logical_and_expresson.m_rest {
            let e2_label = &self.generate_label();
            let end_label = &self.generate_label();
            // if e1 true jump to e2 else jump to end
            gen_s.push_str(
                format!(
                    "\tcmpq\t$0, %rax\n\
                \tjne\t{0}\n\
                \tjmp\t{1}\n\
                {0}:\n",
                    &e2_label, &end_label
                )
                .as_str(),
            );
            gen_s.push_str(&self.generate_equality_expression(&next_op));
            // if e2 true set %al to true
            gen_s.push_str(
                format!(
                    "\tcmpq\t$0, %rax\n\
                \tmovq\t$0, %rax\n\
                \tsetne %al\n\
                {0}:",
                    &end_label
                )
                .as_str(),
            );
        }

        return gen_s;
    }

    fn generate_equality_expression(
        &mut self,
        equality_expession: &EqualityExpression,
    ) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(
            &self.generate_relational_expression(&equality_expession.m_first),
        );

        for next_op in &equality_expession.m_rest {
            gen_s.push_str("\tpush\t%rax\n");
            gen_s.push_str(&self.generate_relational_expression(&next_op.1));
            gen_s.push_str("\tpop\t%rcx\n");
            match next_op.0 {
                EqualityOperator::NotEqual => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsetne\t%al\n",
                    );
                }
                EqualityOperator::Equal => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsete\t%al\n",
                    );
                }
            }
        }

        return gen_s;
    }

    fn generate_relational_expression(
        &mut self,
        relational_expression: &RelationalExpression,
    ) -> String {
        let mut gen_s = String::new();
        gen_s.push_str(
            &self.generate_additive_expression(&relational_expression.m_first),
        );

        for next_op in &relational_expression.m_rest {
            gen_s.push_str("\tpush\t%rax\n");
            gen_s.push_str(&self.generate_additive_expression(&next_op.1));
            gen_s.push_str("\tpop\t%rcx\n");
            match next_op.0 {
                RelationalOperator::Less => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsetl\t%al\n",
                    );
                }
                RelationalOperator::LessOrEqual => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsetle\t%al\n",
                    );
                }
                RelationalOperator::Greater => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsetg\t%al\n",
                    );
                }
                RelationalOperator::GreaterOrEqual => {
                    gen_s.push_str(
                        "\tcmpq\t%rax, %rcx\n\
                \tmovq\t$0, %rax\n\
                \tsetge\t%al\n",
                    );
                }
            }
        }
        return gen_s;
    }

    fn generate_additive_expression(
        &mut self,
        additive_expression: &AdditiveExpression,
    ) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(&self.generate_term(&additive_expression.m_first_term));

        for next_op in &additive_expression.m_rest {
            gen_s.push_str("\tpush\t%rax\n");
            gen_s.push_str(&self.generate_term(&next_op.1));
            gen_s.push_str("\tpop\t%rcx\n");
            match next_op.0 {
                AdditiveOperator::Minus => {
                    gen_s.push_str(
                        "\tsubq\t%rax, %rcx\n\
                    \tmovq\t%rcx, %rax\n",
                    ); // calc rax - rcx store in rcx
                }
                AdditiveOperator::Addition => {
                    gen_s.push_str("\taddq\t%rcx, %rax\n"); // calc rcx + rax store in rax
                }
                _ => (),
            }
        }

        return gen_s;
    }

    fn generate_term(&mut self, term: &Term) -> String {
        let mut gen_s = String::new();

        gen_s.push_str(&self.generate_factor(&term.m_first_factor));

        for next_op in &term.m_rest {
            gen_s.push_str("\tpush\t%rax\n");
            gen_s.push_str(&self.generate_factor(&next_op.1));
            gen_s.push_str("\tpop\t%rcx\n");
            match next_op.0 {
                MultiplicativeOperator::Multiplication => {
                    gen_s.push_str("\timul %rcx, %rax\n"); // calc rax * rcx store in rax
                }
                MultiplicativeOperator::Division => {
                    gen_s.push_str(
                        "\tmovq\t%rax, %rbx\n\
                    \tmovq\t%rcx, %rax\n\
                    \tcqo\n\
                    \tidivq\t%rbx\n",
                    ); // mov rax to rbx; mov rcx to rax; sign extend rax to rdx; calc [rdx:rax]/rbx
                       // stores quotient in rax and remainder in rdx // rax / rbx store in rax
                }
                MultiplicativeOperator::Modulo => {
                    gen_s.push_str(
                        "\tmovq\t%rax, %rbx\n\
                    \tmovq\t%rcx, %rax\n\
                    \tcqo\n\
                    \tidivq\t%rbx\n\
                    \tmovq\t%rdx, %rax\n",
                    ); // mov rax to rbx; mov rcx to rax; sign extend rax to rdx; calc [rdx:rax]/rbx
                       // stores quotient in rax and remainder in rdx // rax / rbx store in rax
                }

                _ => (),
            }
        }

        return gen_s;
    }

    fn generate_factor(&mut self, factor: &Factor) -> String {
        let mut gen_s = String::new();

        match factor {
            Factor::Variable { m_var } => {
                let var_offset = self.query_var(m_var).unwrap();
                gen_s.push_str(
                    format!("\tmovq\t{}(%rbp), %rax\n", var_offset).as_str(),
                );
            }
            Factor::Constant { m_value } => {
                gen_s.push_str(
                    format!("\tmovq\t${0}, %rax\n", &m_value).as_str(),
                );
            }
            Factor::UnaryOperation { m_opertator, m_factor } => {
                gen_s.push_str(&self.generate_factor(&m_factor));
                match m_opertator {
                    UnaryOperator::Complement => {
                        let s = "\tnot\t%rax\n";
                        gen_s.push_str(s);
                    }
                    UnaryOperator::Negation => {
                        let s = "\tcmpq\t$0, %rax\n\
                    \tmovq\t$0, %rax\n\
                    \tsete %al\n";
                        gen_s.push_str(s);
                    }
                    UnaryOperator::Minus => {
                        let s = "\tneg\t%rax\n";
                        gen_s.push_str(s);
                    }
                }
            }
            Factor::Braced { m_expression } => {
                gen_s.push_str(&self.generate_expression(m_expression));
            }
        }

        return gen_s;
    }
}
