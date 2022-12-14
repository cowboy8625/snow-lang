// use snowc_error_messages::report;
// use snowc_errors::CResult;
// use snowc_parse::ParserBuilder;
// use snowc_type_checker::type_check;
pub struct Compiler;

// use inkwell::builder::Builder;
// use inkwell::context::Context;
// use inkwell::module::Module;
// use inkwell::passes::PassManager;
// use inkwell::types::BasicMetadataTypeEnum;
// use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, PointerValue};
// use inkwell::{FloatPredicate, OptimizationLevel};
// use std::collections::HashMap;
//
// // trait Visitor<T> {
// //     fn atom(&self) -> T;
// // }
//
// /// Defines the `Expr` compiler.
// pub struct Compiler<'a, 'ctx> {
//     pub context: &'ctx Context,
//     pub builder: &'a Builder<'ctx>,
//     pub fpm: &'a PassManager<FunctionValue<'ctx>>,
//     pub module: &'a Module<'ctx>,
//     pub function: &'a Function,
//
//     variables: HashMap<String, PointerValue<'ctx>>,
//     fn_value_opt: Option<FunctionValue<'ctx>>,
// }
//
// impl<'a, 'ctx> Compiler<'a, 'ctx> {
//     fn compile_atom(&mut self, atom: Atom) -> Result<, &'s static str> {
//     }
//     /// Compiles the specified `Expr` into an LLVM `FloatValue`.
//     fn compile_expr(&mut self, expr: &Expr) -> Result<FloatValue<'ctx>, &'static str> {
//         match *expr {
//             Expr::Atom(atom) => self.atom(),
//             _ => unimplemented!("{expr:?}"),
//             // Expr::Binary {
//             //     op,
//             //     ref left,
//             //     ref right,
//             // } => {
//             //     if op == '=' {
//             //         // handle assignement
//             //         let var_name = match *left.borrow() {
//             //             Expr::Variable(ref var_name) => var_name,
//             //             _ => {
//             //                 return Err("Expected variable as left-hand operator of assignement.");
//             //             },
//             //         };
//             //
//             //         let var_val = self.compile_expr(right)?;
//             //         let var = self.variables.get(var_name.as_str()).ok_or("Undefined variable.")?;
//             //
//             //         self.builder.build_store(*var, var_val);
//             //
//             //         Ok(var_val)
//             //     } else {
//             //         let lhs = self.compile_expr(left)?;
//             //         let rhs = self.compile_expr(right)?;
//             //
//             //         match op {
//             //             '+' => Ok(self.builder.build_float_add(lhs, rhs, "tmpadd")),
//             //             '-' => Ok(self.builder.build_float_sub(lhs, rhs, "tmpsub")),
//             //             '*' => Ok(self.builder.build_float_mul(lhs, rhs, "tmpmul")),
//             //             '/' => Ok(self.builder.build_float_div(lhs, rhs, "tmpdiv")),
//             //             '<' => Ok({
//             //                 let cmp = self
//             //                     .builder
//             //                     .build_float_compare(FloatPredicate::ULT, lhs, rhs, "tmpcmp");
//             //
//             //                 self.builder
//             //                     .build_unsigned_int_to_float(cmp, self.context.f64_type(), "tmpbool")
//             //             }),
//             //             '>' => Ok({
//             //                 let cmp = self
//             //                     .builder
//             //                     .build_float_compare(FloatPredicate::ULT, rhs, lhs, "tmpcmp");
//             //
//             //                 self.builder
//             //                     .build_unsigned_int_to_float(cmp, self.context.f64_type(), "tmpbool")
//             //             }),
//             //
//             //             custom => {
//             //                 let mut name = String::from("binary");
//             //
//             //                 name.push(custom);
//             //
//             //                 match self.get_function(name.as_str()) {
//             //                     Some(fun) => {
//             //                         match self
//             //                             .builder
//             //                             .build_call(fun, &[lhs.into(), rhs.into()], "tmpbin")
//             //                             .try_as_basic_value()
//             //                             .left()
//             //                         {
//             //                             Some(value) => Ok(value.into_float_value()),
//             //                             None => Err("Invalid call produced."),
//             //                         }
//             //                     },
//             //
//             //                     None => Err("Undefined binary operator."),
//             //                 }
//             //             },
//             //         }
//             //     }
//             // },
//             //
//             // Expr::Call { ref fn_name, ref args } => match self.get_function(fn_name.as_str()) {
//             //     Some(fun) => {
//             //         let mut compiled_args = Vec::with_capacity(args.len());
//             //
//             //         for arg in args {
//             //             compiled_args.push(self.compile_expr(arg)?);
//             //         }
//             //
//             //         let argsv: Vec<BasicMetadataValueEnum> =
//             //             compiled_args.iter().by_ref().map(|&val| val.into()).collect();
//             //
//             //         match self
//             //             .builder
//             //             .build_call(fun, argsv.as_slice(), "tmp")
//             //             .try_as_basic_value()
//             //             .left()
//             //         {
//             //             Some(value) => Ok(value.into_float_value()),
//             //             None => Err("Invalid call produced."),
//             //         }
//             //     },
//             //     None => Err("Unknown function."),
//             // },
//             //
//             // Expr::Conditional {
//             //     ref cond,
//             //     ref consequence,
//             //     ref alternative,
//             // } => {
//             //     let parent = self.fn_value();
//             //     let zero_const = self.context.f64_type().const_float(0.0);
//             //
//             //     // create condition by comparing without 0.0 and returning an int
//             //     let cond = self.compile_expr(cond)?;
//             //     let cond = self
//             //         .builder
//             //         .build_float_compare(FloatPredicate::ONE, cond, zero_const, "ifcond");
//             //
//             //     // build branch
//             //     let then_bb = self.context.append_basic_block(parent, "then");
//             //     let else_bb = self.context.append_basic_block(parent, "else");
//             //     let cont_bb = self.context.append_basic_block(parent, "ifcont");
//             //
//             //     self.builder.build_conditional_branch(cond, then_bb, else_bb);
//             //
//             //     // build then block
//             //     self.builder.position_at_end(then_bb);
//             //     let then_val = self.compile_expr(consequence)?;
//             //     self.builder.build_unconditional_branch(cont_bb);
//             //
//             //     let then_bb = self.builder.get_insert_block().unwrap();
//             //
//             //     // build else block
//             //     self.builder.position_at_end(else_bb);
//             //     let else_val = self.compile_expr(alternative)?;
//             //     self.builder.build_unconditional_branch(cont_bb);
//             //
//             //     let else_bb = self.builder.get_insert_block().unwrap();
//             //
//             //     // emit merge block
//             //     self.builder.position_at_end(cont_bb);
//             //
//             //     let phi = self.builder.build_phi(self.context.f64_type(), "iftmp");
//             //
//             //     phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);
//             //
//             //     Ok(phi.as_basic_value().into_float_value())
//             // },
//         }
//     }
//
//     /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
//     pub fn new(
//         context: &'ctx Context,
//         builder: &'a Builder<'ctx>,
//         pass_manager: &'a PassManager<FunctionValue<'ctx>>,
//         module: &'a Module<'ctx>,
//         function: &Function,
//     ) -> Result<FunctionValue<'ctx>, &'static str> {
//         let mut compiler = Compiler {
//             context,
//             builder,
//             fpm: pass_manager,
//             module,
//             function,
//             fn_value_opt: None,
//             variables: HashMap::new(),
//         };
//
//         compiler.compile_fn()
//     }
// }
