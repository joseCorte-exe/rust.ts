use crate::ast::{BinaryOperator, Expression, Statement, Type};
use std::fs;
use std::path::Path;

pub struct CodeGenerator {
    output_dir: String,
}

impl CodeGenerator {
    pub fn new(output_dir: &str) -> Self {
        CodeGenerator {
            output_dir: output_dir.to_string(),
        }
    }

    pub fn generate(&self, statements: Vec<Statement>) -> Result<(), std::io::Error> {
        // Cria o diretório de saída se não existir
        fs::create_dir_all(&self.output_dir)?;

        // Cria o Cargo.toml
        let cargo_toml = r#"[package]
name = "generated-code"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
        fs::write(Path::new(&self.output_dir).join("Cargo.toml"), cargo_toml)?;

        let mut code = String::new();
        code.push_str("fn main() {\n");

        for stmt in statements {
            code.push_str(&self.generate_statement(&stmt));
        }

        code.push_str("}\n");

        // Escreve o código no arquivo
        let output_path = Path::new(&self.output_dir).join("src/main.rs");
        fs::create_dir_all(Path::new(&self.output_dir).join("src"))?;
        fs::write(output_path, code)?;

        Ok(())
    }

    fn generate_statement(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::ConsoleLog(expr) => {
                format!(
                    "    println!(\"{{:?}}\", {});\n",
                    self.generate_expression(expr)
                )
            }
            Statement::VariableDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let type_str = match type_annotation {
                    Type::String => "String",
                    Type::Number => "i32",
                    Type::Boolean => "bool",
                };

                let value_str = if let Some(expr) = value {
                    format!(" = {}", self.generate_expression(expr))
                } else {
                    String::new()
                };

                let mut_str = if name == "contador" { "mut " } else { "" };

                format!("    let {}{}: {}{};\n", mut_str, name, type_str, value_str)
            }

            Statement::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut code = format!("    if {} {{\n", self.generate_expression(condition));

                for stmt in then_branch {
                    code.push_str(&self.generate_statement(stmt));
                }

                code.push_str("    }");

                if let Some(else_statements) = else_branch {
                    code.push_str(" else {\n");
                    for stmt in else_statements {
                        code.push_str(&self.generate_statement(stmt));
                    }
                    code.push_str("    }");
                }

                code.push_str("\n");
                code
            }
            Statement::WhileStatement { condition, body } => {
                let mut code = format!("    while {} {{\n", self.generate_expression(condition));
                for stmt in body {
                    code.push_str(&self.generate_statement(stmt));
                }
                code.push_str("    }\n");
                code
            }
            Statement::Assignment { name, value } => {
                format!("    {} = {};\n", name, self.generate_expression(value))
            }
        }
    }

    fn generate_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::StringLiteral(s) => {
                let s = s.trim_matches('"');
                let escaped = s.replace('"', "\\\"");
                format!("String::from(\"{}\")", escaped)
            }

            Expression::NumberLiteral(n) => n.to_string(),
            Expression::Identifier(name) => name.clone(),
            // 👇 Adicione isso:
            Expression::BinaryOp { left, op, right } => {
                let left_is_string = matches!(**left, Expression::StringLiteral(_));
                let right_is_identifier_or_number = matches!(
                    **right,
                    Expression::Identifier(_) | Expression::NumberLiteral(_)
                );

                if *op == BinaryOperator::Add && left_is_string && right_is_identifier_or_number {
                    // Detectamos concatenação com string à esquerda: usar format!
                    let left_str = if let Expression::StringLiteral(s) = &**left {
                        s.trim_matches('"').to_string()
                    } else {
                        "".to_string()
                    };

                    let right_str = self.generate_expression(right);
                    return format!(
                        "format!(\"{}{{}}\", {})",
                        left_str.replace('"', "\\\""),
                        right_str
                    );
                }

                // caso padrão
                let left_code = self.generate_expression(left);
                let right_code = self.generate_expression(right);
                let op_code = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::LessThan => "<",
                    BinaryOperator::GreaterThan => ">",
                };
                format!("{} {} {}", left_code, op_code, right_code)
            }
            Expression::Assignment { name, value } => {
                format!("{} = {}", name, self.generate_expression(value))
            }
        }
    }
}
