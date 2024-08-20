use super::{
    module::Module,
    opcode::Instruction,
    section::{
        code::{Block, Code},
        data::{Data, Segment},
        export::{Export, ExportEntry, ExportType},
        function::Function,
        header::Header,
        import::{Import, ImportEntry, ImportType},
        memory::{Memory, Page},
        start::Start,
        DataType, Section,
        _type::{FunctionType, Type, ValueType},
    },
};
use crate::ir;

#[derive(Debug)]
pub struct Emitter {
    module: Module,
    ir_module: ir::Module,
}

impl Emitter {
    pub fn new(ir_module: ir::Module) -> Self {
        Self {
            module: Module::default(),
            ir_module,
        }
    }

    fn compile_value(
        &mut self,
        value: &ir::Value,
        params: &[(String, ir::Type)],
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match value {
            ir::Value::Int(value) => instructions.push(Instruction::I32Const(*value)),
            ir::Value::Char(value) => {
                instructions.push(Instruction::I32Const(*value as i32))
            }
            ir::Value::Bool(value) => {
                instructions.push(Instruction::I32Const(*value as i32))
            }
            ir::Value::Variable(name) => {
                let variable_name = params
                    .iter()
                    .position(|(param, _)| param == name)
                    .map(|index| index as u32)
                    .or(self.module.get_function_id(name));
                let Some(index) = variable_name else {
                    panic!("Variable not found: {}", name);
                };
                instructions.push(Instruction::LocalGet(index))
            }
            ir::Value::String(string) => {
                let ptr = self.module.add_string(string);
                instructions.push(Instruction::I32Const(ptr as i32));
                instructions.push(Instruction::I32Const(string.len() as i32));
            }
        }
        instructions
    }

    fn compile_instruction(
        &mut self,
        instruction: &ir::Instruction,
        params: &[(String, ir::Type)],
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match instruction {
            ir::Instruction::If {
                condition,
                then_branch,
                else_branch,
            } => {
                instructions.extend(self.compile_block(condition, params));
                instructions.push(Instruction::If(DataType::I32));
                instructions.extend(self.compile_block(then_branch, params));
                if let Some(else_branch) = else_branch {
                    instructions.push(Instruction::Else);
                    instructions.extend(self.compile_block(else_branch, params));
                }
                instructions.push(Instruction::End);
            }
            ir::Instruction::Call {
                function_name,
                arguments,
            } => todo!(),
            ir::Instruction::Return(_) => todo!(),
            ir::Instruction::Add => todo!(),
            ir::Instruction::Sub => todo!(),
            ir::Instruction::Mul => todo!(),
            ir::Instruction::Div => todo!(),
            ir::Instruction::GreaterThan => instructions.push(Instruction::I32Gt),
            ir::Instruction::Load(_) => todo!(),
            ir::Instruction::Store(_, _) => todo!(),
            ir::Instruction::Push(value) => {
                let values = self.compile_value(value, params);
                instructions.extend(values);
            }
            ir::Instruction::Pop(_) => todo!(),
        }
        instructions
    }

    fn compile_block(
        &mut self,
        expr: &ir::Block,
        params: &[(String, ir::Type)],
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        for instruction in &expr.instructions {
            instructions.extend(self.compile_instruction(instruction, params));
        }
        instructions
    }

    fn compile_function_in_module(&mut self) {
        let get_type = |ty: &ir::Type| match ty {
            ir::Type::Int => DataType::I32,
            _ => todo!(),
        };
        for func in self.ir_module.functions.clone().into_iter() {
            let ir::Function {
                name,
                params,
                return_type,
                body,
            } = func;

            let mut func_type = FunctionType::default();
            for (pname, ty) in params.iter() {
                let value_type = ValueType::WithName(pname.to_string(), get_type(ty));
                func_type = func_type.with_param(value_type);
            }

            if let Some(return_type) = return_type {
                func_type = func_type.with_result(get_type(&return_type));
            }

            let mut block_instructions = self.compile_block(&body, &params);
            block_instructions.push(Instruction::Drop);
            let block = Block::new(block_instructions);

            self.module.add_function(name, func_type, block);
        }
    }

    pub fn emit(mut self) -> Module {
        // self.module.add_memory(Page::WithNoMinimun(1));
        self.module.import(
            "core",
            "write",
            FunctionType::default()
                .with_param(ValueType::Data(DataType::I32))
                .with_param(ValueType::Data(DataType::I32))
                .with_result(DataType::I32),
        );

        self.compile_function_in_module();

        let Some(main_id) = self.module.get_main_function_id() else {
            panic!("No main function found");
        };
        self.module.set_start(main_id);

        self.module
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_end::{Parser, Token};
    use crate::ir::{self, ExprVisitor};
    use logos::Logos;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_wasm_emitter() {
        let input = r#"
max x y
    : Int -> Int -> Int
    = if x > y then x else y
            "#;
        let lexer = Token::lexer(input);
        let mut parser = Parser::new(lexer.peekable());
        let ast = parser.parse().unwrap();

        let mut ir_emitter = ir::Emitter::new();
        let ir_module = ir_emitter.visit(&ast);
        eprintln!("ir_module: {:#?}", ir_module);
        let mut wasm_emitter = Emitter::new(ir_module);
        let wasm_module = wasm_emitter.emit();
        assert_eq!(
            wasm_module
                .function
                .as_ref()
                .map(|f| f.to_bytes().unwrap_or_default().len()),
            Some(4)
        );
        assert_eq!(
            wasm_module.function,
            Some(Function::default().with_function("max"))
        );
        assert_eq!(
            wasm_module.code,
            Some(
                Code::default().with(
                    Block::default()
                        .with(Instruction::LocalGet(0))
                        .with(Instruction::LocalGet(1))
                        .with(Instruction::I32Gt)
                        .with(Instruction::If(DataType::I32))
                        .with(Instruction::LocalGet(0))
                        .with(Instruction::Else)
                        .with(Instruction::LocalGet(1))
                        .with(Instruction::End)
                )
            )
        );
    }
}
