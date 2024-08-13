use crate::front_end::*;
use crate::ir::*;

pub struct IrEmitter {
    functions: Vec<IRFunction>,
    enums: Vec<IREnum>,
}

impl IrEmitter {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            enums: Vec::new(),
        }
    }

    pub fn visit(&mut self, expr: &Vec<Expr>) -> IRModule {
        for e in expr {
            match e {
                // Expr::Atom(atom) => todo!(),
                // Expr::Identifier(string) => todo!(),
                // Expr::PrefixOp { op, right } => todo!(),
                // Expr::BinaryOp { left, op, right } => todo!(),
                // Expr::If {
                //     condition,
                //     then_branch,
                //     else_branch,
                // } => todo!(),
                Expr::Function {
                    name,
                    params,
                    signature,
                    body,
                } => {
                    let function = self.visit_function(
                        name.clone(),
                        params.clone(),
                        signature.clone(),
                        body,
                    );
                    self.functions.push(function);
                }
                Expr::Enum { name, variants, .. } => {
                    let enum_ = self.visit_enum(name.clone(), Vec::new(), variants);
                    self.enums.push(enum_);
                }
                _ => unreachable!(),
            }
        }
        IRModule {
            functions: self.functions.clone(),
            globals: Vec::new(),
            types: Vec::new(),
            enums: self.enums.clone(),
        }
    }
}

impl ExprVisitor for IrEmitter {
    fn visit_function(
        &mut self,
        name: String,
        params: Vec<String>,
        signature: Vec<String>,
        body: &Expr,
    ) -> IRFunction {
        let ir_params = params
            .iter()
            .zip(signature[..params.len() - 1].to_vec())
            .map(|(n, s)| (n.clone(), IRType::from(s)))
            .collect();
        IRFunction {
            name,
            params: ir_params,
            return_type: IRType::from(&signature[signature.len() - 1]),
            body: self.visit_expr(body),
        }
    }

    fn visit_enum(
        &mut self,
        name: String,
        type_args: Vec<String>,
        variants: &Vec<Vec<Expr>>,
    ) -> IREnum {
        todo!()
    }

    fn visit_enum_variant(&mut self, variant: &Vec<Expr>) -> IREnumVariant {
        todo!()
    }

    fn visit_binary_op(
        &mut self,
        left: &Expr,
        op: Operator,
        right: &Expr,
    ) -> IRInstruction {
        todo!()
    }

    fn visit_prefix_op(&mut self, op: Operator, right: &Expr) -> IRInstruction {
        todo!()
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Option<Expr>,
    ) -> IRInstruction {
        todo!()
    }
}
