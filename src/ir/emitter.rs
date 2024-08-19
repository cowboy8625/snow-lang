use crate::front_end::{Atom, Expr, Operator};
use crate::ir::{
    code::{Block, Function, Instruction, Module, Type, Value},
    expr_visitor::ExprVisitor,
};

#[derive(Debug)]
pub struct Emitter {
    functions: Vec<Function>,
    instructions: Vec<Instruction>,
    blocks: Vec<Block>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            blocks: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn get_block(&mut self) -> Block {
        Block {
            instructions: std::mem::take(&mut self.instructions),
        }
    }
}

impl ExprVisitor for Emitter {
    fn visit_atom(&mut self, atom: &Atom) {
        let value = match atom {
            Atom::Int(value) => Value::Int(*value),
            Atom::Char(value) => Value::Char(*value),
            Atom::Bool(value) => Value::Bool(*value),
        };

        self.instructions.push(Instruction::Push(value));
    }

    fn visit_binary_op(&mut self, left: &Expr, op: &Operator, right: &Expr) {
        let operation_instruction = match op {
            Operator::Add => todo!(),
            Operator::Sub => todo!(),
            Operator::Mul => todo!(),
            Operator::Div => todo!(),
            Operator::Mod => todo!(),
            Operator::And => todo!(),
            Operator::Or => todo!(),
            Operator::Not => todo!(),
            Operator::Equal => todo!(),
            Operator::NotEqual => todo!(),
            Operator::GreaterThan => Instruction::GreaterThan,
            Operator::GreaterThanOrEqual => todo!(),
            Operator::LessThan => todo!(),
            Operator::LessThanOrEqual => todo!(),
        };

        self.visit_expr(left);
        self.visit_expr(right);
        self.instructions.push(operation_instruction);
    }

    fn visit_if(
        self: &mut Self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: Option<Box<Expr>>,
    ) {
        self.visit_expr(condition);
        let condition_block = self.get_block();

        eprintln!("then branch: {:?}", then_branch);
        self.visit_expr(then_branch);
        let then_branch_block = self.get_block();

        let else_branch_block = if let Some(else_branch) = else_branch {
            self.visit_expr(else_branch.as_ref());
            let block = self.get_block();
            Some(block)
        } else {
            None
        };

        self.instructions.push(Instruction::If {
            condition: condition_block,
            then_branch: then_branch_block,
            else_branch: else_branch_block,
        })
    }

    fn visit_function(
        &mut self,
        name: &str,
        params: &[String],
        signature: &[String],
        body: &Expr,
    ) {
        self.visit_expr(body);
        let block = self.get_block();
        let func = Function {
            name: name.to_string(),
            params: params
                .iter()
                .zip(signature[0..params.len()].iter())
                .map(|(name, ty)| (name.to_string(), ty.into()))
                .collect(),
            return_type: signature.last().map(|ty| ty.into()),
            body: block,
        };

        self.functions.push(func);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Atom(atom) => self.visit_atom(atom),
            Expr::Identifier(name) => {
                // TODO: Move Identifier to Atom enum
                let value = Value::Variable(name.to_string());
                self.instructions.push(Instruction::Push(value));
            }
            Expr::PrefixOp { op, right } => todo!(),
            Expr::BinaryOp { left, op, right } => self.visit_binary_op(left, op, right),
            Expr::Function {
                name,
                params,
                signature,
                body,
            } => self.visit_function(name, params, signature, body),
            Expr::Enum {
                name,
                type_args,
                variants,
            } => todo!(),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => self.visit_if(condition, then_branch, else_branch.clone()),
        }
    }

    fn visit(&mut self, expressions: &[Expr]) -> Module {
        for expr in expressions {
            self.visit_expr(expr);
        }

        Module {
            functions: self.functions.clone(),
            globals: vec![],
            enums: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_end::{Parser, Token};
    use logos::Logos;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_emit_ir_function() {
        let input = r#"
max x y
    : Int -> Int -> Int
    = if x > y then x else y
            "#;
        let lexer = Token::lexer(input);
        let mut parser = Parser::new(lexer.peekable());
        let ast = parser.parse().unwrap();

        let mut emitter = Emitter::new();
        let module = emitter.visit(&ast);

        eprintln!("{:#?}", emitter);
        eprintln!("{:#?}", module);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].name, "max");
        assert_eq!(
            module.functions[0].params,
            vec![("x".to_string(), Type::Int), ("y".to_string(), Type::Int)]
        );
        assert_eq!(module.functions[0].return_type, Some(Type::Int));
        assert_eq!(
            module.functions[0].body,
            Block {
                instructions: vec![Instruction::If {
                    condition: Block {
                        instructions: vec![
                            Instruction::Push(Value::Variable("x".to_string())),
                            Instruction::Push(Value::Variable("y".to_string())),
                            Instruction::GreaterThan,
                        ],
                    },
                    then_branch: Block {
                        instructions: vec![Instruction::Push(Value::Variable(
                            "x".to_string(),
                        ))]
                    },
                    else_branch: Some(Block {
                        instructions: vec![Instruction::Push(Value::Variable(
                            "y".to_string(),
                        ))]
                    }),
                }]
            }
        );
    }

    //     #[test]
    //     fn test_emit_ir_enum() {
    //         let input = r#"
    //             enum Option a
    //                 = Some a
    //                 | None
    //             "#;
    //         let lexer = Token::lexer(input);
    //         let mut parser = Parser::new(lexer.peekable());
    //         let ast = parser.parse().unwrap();

    //         let mut emitter = Emitter::new();
    //         let module = emitter.visit(&ast);

    //         eprintln!("{:#?}", emitter);
    //         eprintln!("{:#?}", module);
    //         assert_eq!(module.enums.len(), 1);
    //     }
}
