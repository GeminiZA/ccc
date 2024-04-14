use crate::parser::Expression;
use crate::parser::Function;
use crate::parser::Operator;
use crate::parser::Program;
use crate::parser::Statement;

fn generate_expression(expression: &Expression) -> String {
    let mut gen_s = String::new();

    match expression {
        Expression::Constant { m_value } => {
            gen_s.push_str(format!("movq\t${0}, %rax\n", m_value).as_str())
        }
        Expression::OperUnary { m_operator, m_value } => {
            gen_s.push_str(&generate_expression(&m_value));
            match m_operator {
                Operator::OperatorComplement => {
                    let s = "not\t%rax\n";
                    gen_s.push_str(s);
                }
                Operator::OperatorMinus => {
                    let s = "neg\t%rax\n";
                    gen_s.push_str(s);
                }
                Operator::OperatorNegation => {
                    let s = "cmpq\t$0, %rax\n\
                    movq\t$0, %rax\n\
                    sete %al\n";
                    gen_s.push_str(s);
                }
            };
        }
    }

    return gen_s;
}

fn generate_statment(statement: &Statement) -> String {
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

    gen_s.push_str(generate_statment(&function.m_statement).as_str());

    return gen_s;
}

pub fn generate(program: &Program) -> String {
    let mut gen_s: String = String::new();

    gen_s.push_str(&generate_function(&program.m_function));

    return gen_s;
}
