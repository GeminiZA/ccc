use crate::parser::{
    BinaryOperator, Expression, Factor, Function, Program, Statement, Term,
    UnaryOperator,
};

fn generate_expression(expression: &Expression) -> String {
    let mut gen_s = String::new();

    gen_s.push_str(&generate_term(&expression.m_first_term));

    for next_op in &expression.m_rest {
        gen_s.push_str("push %rax\n");
        gen_s.push_str(&generate_term(&next_op.1));
        gen_s.push_str("pop %rcx\n");
        match next_op.0 {
            BinaryOperator::Minus => {
                gen_s.push_str(
                    "subq\t%rax, %rcx\n\
                    movq\t%rcx, %rax\n",
                ); // calc rax - rcx store in rcx
            }
            BinaryOperator::Addition => {
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
            BinaryOperator::Multiplication => {
                gen_s.push_str("imul %rcx, %rax\n"); // calc rax * rcx store in rax
            }
            BinaryOperator::Division => {
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
