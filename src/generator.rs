use crate::parser;
use crate::parser::Expression;
use crate::parser::Operator;
use crate::parser::Statement;

pub fn generate(program: &parser::Program) -> String {
    let mut gen_s: String = String::new();

    let func = &program.m_function;

    gen_s.push_str(format!(".globl {0}\n{0}:\n", &func.m_id).as_str());

    let statem = &func.m_statement;

    match statem {
        Statement::Return(expr) => match expr {
            Expression::Constant { m_value } => {
                gen_s.push_str(
                    format!("mov\t${0}, %rax\nret", m_value).as_str(),
                );
            }
            Expression::OperUnary { m_operator, m_value } => match m_operator {
                Operator::OperatorMinus => gen_s.push_str(format!("").as_str()),
                Operator::OperatorNegation => gen_s
                    .push_str(format!("movl\t${0}, %rax\n", m_value).as_str()),
                Operator::OperatorComplement => {
                    gen_s.push_str(format!("").as_str())
                }
            },
            _ => (),
        },
        _ => (),
    };

    gen_s.push('\n');

    return gen_s;
}
