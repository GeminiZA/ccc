use std::panic::Location;

use crate::parser::{
    AdditiveExpression, AdditiveOperator, EqualityExpression, EqualityOperator,
    Expression, Factor, Function, LogicalAndExpression, MultiplicativeOperator,
    Program, RelationalExpression, RelationalOperator, Statement, Term,
    UnaryOperator,
};

static mut LABEL_NUMBER: i32 = 0;

fn generate_label() -> String {
    unsafe {
        let label = format!("label_{}", LABEL_NUMBER);
        LABEL_NUMBER += 1;
        return label;
    }
}

fn generate_expression(expression: &Expression) -> String {
    let mut gen_s = String::new();

    gen_s.push_str(&generate_logical_and_expression(&expression.m_first));

    for next_op in &expression.m_rest {
        let e2_label = &generate_label();
        let end_label = &generate_label();
        gen_s.push_str(
            format!(
                "cmpq\t$0, %rax\n\
                je\t{0}\n\
                movq\t$1, %rax\n\
                jmp\t{1}\n\
                {0}:\n",
                &e2_label, &end_label,
            )
            .as_str(),
        );
        gen_s.push_str(&generate_logical_and_expression(&next_op));
        gen_s.push_str(
            format!(
                "cmpq\t$0, %rax\n\
                movq\t$0, %rax\n\
                setne\t%al\n\
                {0}:",
                &end_label
            )
            .as_str(),
        );
    }

    return gen_s;
}

fn generate_logical_and_expression(
    logical_and_expresson: &LogicalAndExpression,
) -> String {
    let mut gen_s = String::new();

    gen_s.push_str(&generate_equality_expression(
        &logical_and_expresson.m_first,
    ));

    for next_op in &logical_and_expresson.m_rest {
        let e2_label = &generate_label();
        let end_label = &generate_label();
        // if e1 true jump to e2 else jump to end
        gen_s.push_str(
            format!(
                "cmpq\t$0, %rax\n\
                jne\t{0}\n\
                jmp\t{1}\n\
                {0}:\n",
                &e2_label, &end_label
            )
            .as_str(),
        );
        gen_s.push_str(&generate_equality_expression(&next_op));
        // if e2 true set %al to true
        gen_s.push_str(
            format!(
                "cmpq\t$0, %rax\n\
                movq\t$0, %rax\n\
                setne %al\n\
                {0}:",
                &end_label
            )
            .as_str(),
        );
    }

    return gen_s;
}

fn generate_equality_expression(
    equality_expession: &EqualityExpression,
) -> String {
    let mut gen_s = String::new();

    gen_s
        .push_str(&generate_relational_expression(&equality_expession.m_first));

    for next_op in &equality_expession.m_rest {
        gen_s.push_str("push %rax\n");
        gen_s.push_str(&generate_relational_expression(&next_op.1));
        gen_s.push_str("pop %rcx\n");
        match next_op.0 {
            EqualityOperator::NotEqual => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                setne\t%al\n",
                );
            }
            EqualityOperator::Equal => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                sete\t%al\n",
                );
            }
        }
    }

    return gen_s;
}

fn generate_relational_expression(
    relational_expression: &RelationalExpression,
) -> String {
    let mut gen_s = String::new();
    gen_s.push_str(&generate_additive_expression(
        &relational_expression.m_first,
    ));

    for next_op in &relational_expression.m_rest {
        gen_s.push_str("push %rax\n");
        gen_s.push_str(&generate_additive_expression(&next_op.1));
        gen_s.push_str("pop %rcx\n");
        match next_op.0 {
            RelationalOperator::Less => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                setl\t%al\n",
                );
            }
            RelationalOperator::LessOrEqual => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                setle\t%al\n",
                );
            }
            RelationalOperator::Greater => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                setg\t%al\n",
                );
            }
            RelationalOperator::GreaterOrEqual => {
                gen_s.push_str(
                    "cmpq\t%rax, %rcx\n\
                movq\t$0, %rax\n\
                setge\t%al\n",
                );
            }
        }
    }
    return gen_s;
}

fn generate_additive_expression(
    additive_expression: &AdditiveExpression,
) -> String {
    let mut gen_s = String::new();

    gen_s.push_str(&generate_term(&additive_expression.m_first_term));

    for next_op in &additive_expression.m_rest {
        gen_s.push_str("push %rax\n");
        gen_s.push_str(&generate_term(&next_op.1));
        gen_s.push_str("pop %rcx\n");
        match next_op.0 {
            AdditiveOperator::Minus => {
                gen_s.push_str(
                    "subq\t%rax, %rcx\n\
                    movq\t%rcx, %rax\n",
                ); // calc rax - rcx store in rcx
            }
            AdditiveOperator::Addition => {
                gen_s.push_str("addq\t%rcx, %rax\n"); // calc rcx + rax store in rax
            }
            _ => (),
        }
    }

    return gen_s;
}

fn generate_term(term: &Term) -> String {
    let mut gen_s = String::new();

    gen_s.push_str(&generate_factor(&term.m_first_factor));

    for next_op in &term.m_rest {
        gen_s.push_str("push\t%rax\n");
        gen_s.push_str(&generate_factor(&next_op.1));
        gen_s.push_str("pop\t%rcx\n");
        match next_op.0 {
            MultiplicativeOperator::Multiplication => {
                gen_s.push_str("imul %rcx, %rax\n"); // calc rax * rcx store in rax
            }
            MultiplicativeOperator::Division => {
                gen_s.push_str(
                    "movq\t%rax, %rbx\n\
                    movq\t%rcx, %rax\n\
                    cdq\n\
                    idivq\t%rbx\n",
                ); // mov rax to rbx; mov rcx to rax; sign extend rax to rdx; calc [rdx:rax]/rbx
                   // stores quotient in rax and remainder in rdx // rax / rbx store in rax
            }
            _ => (),
        }
    }

    return gen_s;
}

fn generate_factor(factor: &Factor) -> String {
    let mut gen_s = String::new();

    match factor {
        Factor::Constant { m_value } => {
            gen_s.push_str(format!("movq\t${0}, %rax\n", &m_value).as_str());
        }
        Factor::UnaryOperation { m_opertator, m_factor } => {
            gen_s.push_str(&generate_factor(&m_factor));
            match m_opertator {
                UnaryOperator::Complement => {
                    let s = "not\t%rax\n";
                    gen_s.push_str(s);
                }
                UnaryOperator::Negation => {
                    let s = "cmpq\t%rax\n\
                    movq\t$0, %rax\n\
                    sete %al\n";
                    gen_s.push_str(s);
                }
                UnaryOperator::Minus => {
                    let s = "neg\t%rax\n";
                    gen_s.push_str(s);
                }
            }
        }
        Factor::Braced { m_expression } => {
            gen_s.push_str(&generate_expression(m_expression));
        }
    }

    return gen_s;
}

fn generate_statement(statement: &Statement) -> String {
    let mut gen_s: String = String::new();

    match statement {
        Statement::Return(expr) => {
            gen_s.push_str(&generate_expression(expr).as_str());
            gen_s.push_str(format!("ret\n").as_str());
        }
    }

    return gen_s;
}

fn generate_function(function: &Function) -> String {
    let mut gen_s: String = String::new();

    gen_s.push_str(format!(".globl {0}\n{0}:\n", function.m_id).as_str());

    match &function.m_statement {
        Some(s) => gen_s.push_str(generate_statement(s).as_str()),
        None => (),
    }

    return gen_s;
}

pub fn generate(program: &Program) -> String {
    let mut gen_s: String = String::new();

    gen_s.push_str(&generate_function(&program.m_function));

    return gen_s;
}
