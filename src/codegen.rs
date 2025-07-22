//! Code generation for the strongly typed TypeScript compiler
//!
//! This module converts the type-checked AST into LLVM IR and handles
//! compilation to native code via LLVM.

use crate::ast::ConsoleMethod;
use crate::ast::*;
use crate::error::{DrafError, DrafResult};
use crate::semantic::{TypedExpression, TypedProgram, TypedStatement};
use crate::types::Type;
use inkwell::builder::Builder;
use inkwell::context::Context;

use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue,
};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Code generator that converts typed AST to LLVM IR
pub struct CodeGenerator<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// LLVM module
    module: Module<'ctx>,
    /// LLVM builder
    builder: Builder<'ctx>,
    /// Symbol table for variables
    variables: HashMap<String, PointerValue<'ctx>>,
    /// Type information for variables
    variable_types: HashMap<String, Type>,
    /// Current function being generated
    current_function: Option<FunctionValue<'ctx>>,
    /// Printf function for console output
    printf_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    /// Create a new code generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        let mut codegen = CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            variable_types: HashMap::new(),
            current_function: None,
            printf_function: None,
        };

        // Declare printf function for console output
        codegen.declare_printf();
        codegen
    }

    /// Declare the printf function for console output
    fn declare_printf(&mut self) {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let printf_type = self.context.i32_type().fn_type(&[i8_ptr_type.into()], true);
        let printf_fn = self.module.add_function("printf", printf_type, None);
        self.printf_function = Some(printf_fn);
    }

    /// Generate LLVM IR for a typed program
    pub fn generate(&mut self, program: TypedProgram) -> DrafResult<()> {
        // Create main function
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");

        self.current_function = Some(main_fn);
        self.builder.position_at_end(main_block);

        // Generate code for each statement
        for statement in program.statements {
            self.generate_statement(statement)?;
        }

        // Return 0 from main
        let zero = self.context.i32_type().const_int(0, false);
        self.builder.build_return(Some(&zero)).unwrap();

        // Verify the module
        if let Err(errors) = self.module.verify() {
            return Err(DrafError::llvm_error(format!(
                "Module verification failed: {}",
                errors.to_string()
            )));
        }

        Ok(())
    }

    /// Generate code for a statement
    fn generate_statement(&mut self, statement: TypedStatement) -> DrafResult<()> {
        match statement {
            TypedStatement::VariableDeclaration {
                name,
                initializer,
                inferred_type,
                location: _,
                ..
            } => {
                let llvm_type = self.type_to_llvm_type(&inferred_type)?;

                // Allocate space for the variable
                let alloca = self.builder.build_alloca(llvm_type, &name).unwrap();
                self.variables.insert(name.clone(), alloca);
                self.variable_types.insert(name.clone(), inferred_type);

                // Generate initializer if present
                if let Some(init_expr) = initializer {
                    let init_value = self.generate_expression(init_expr)?;
                    self.builder.build_store(alloca, init_value).unwrap();
                }

                Ok(())
            }

            TypedStatement::ExpressionStatement { expression, .. } => {
                self.generate_expression(expression)?;
                Ok(())
            }

            TypedStatement::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                // Generate condition
                let condition_val = self.generate_expression(condition)?;
                let condition_bool = condition_val.into_int_value();

                // Get current function
                let current_fn = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                // Create basic blocks
                let then_block = self.context.append_basic_block(current_fn, "then");
                let else_block = self.context.append_basic_block(current_fn, "else");
                let merge_block = self.context.append_basic_block(current_fn, "merge");

                // Build conditional branch
                self.builder
                    .build_conditional_branch(condition_bool, then_block, else_block)
                    .unwrap();

                // Generate then branch
                self.builder.position_at_end(then_block);
                self.generate_statement(*then_branch)?;
                self.builder
                    .build_unconditional_branch(merge_block)
                    .unwrap();

                // Generate else branch
                self.builder.position_at_end(else_block);
                if let Some(else_stmt) = else_branch {
                    self.generate_statement(*else_stmt)?;
                }
                self.builder
                    .build_unconditional_branch(merge_block)
                    .unwrap();

                // Continue with merge block
                self.builder.position_at_end(merge_block);
                Ok(())
            }

            TypedStatement::Block { statements, .. } => {
                for statement in statements {
                    self.generate_statement(statement)?;
                }
                Ok(())
            }
        }
    }

    /// Generate code for an expression
    fn generate_expression(&mut self, expr: TypedExpression) -> DrafResult<BasicValueEnum<'ctx>> {
        match expr.expression {
            Expression::Literal { value, .. } => match value {
                LiteralValue::Number(n) => Ok(self.context.f64_type().const_float(n).into()),
                LiteralValue::String(_s) => {
                    // For now, strings are not fully implemented
                    Err(DrafError::codegen_error(
                        "String literals not yet implemented in codegen",
                    ))
                }
                LiteralValue::Boolean(b) => {
                    let value = if b { 1 } else { 0 };
                    Ok(self.context.bool_type().const_int(value, false).into())
                }
                LiteralValue::Null => Ok(self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null()
                    .into()),
                LiteralValue::Undefined => Ok(self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null()
                    .into()),
            },

            Expression::Identifier { name, location: _ } => {
                if let Some(var_ptr) = self.variables.get(&name) {
                    // Look up the actual type of the variable
                    let var_type = self.variable_types.get(&name).unwrap_or(&Type::Number);
                    let llvm_type = self.type_to_llvm_type(var_type)?;
                    let loaded_value = self.builder.build_load(llvm_type, *var_ptr, &name).unwrap();
                    Ok(loaded_value)
                } else {
                    Err(DrafError::codegen_error(format!(
                        "Undefined variable in codegen: {}",
                        name
                    )))
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                location: _,
            } => {
                // Use operand types if available, otherwise infer from context
                let (left_type, right_type) = if let Some((left_t, right_t)) = &expr.operand_types {
                    (left_t.clone(), right_t.clone())
                } else {
                    // Fallback: infer types from expressions
                    let left_type = self.infer_expression_type(&left)?;
                    let right_type = self.infer_expression_type(&right)?;
                    (left_type, right_type)
                };

                let typed_left = TypedExpression::new(*left, left_type.clone());
                let typed_right = TypedExpression::new(*right, right_type.clone());
                let left_val = self.generate_expression(typed_left)?;
                let right_val = self.generate_expression(typed_right)?;

                // Handle operations based on operand types, not result type
                match (&left_type, &right_type, &operator) {
                    (Type::Number, Type::Number, BinaryOperator::Add)
                    | (Type::Number, Type::Number, BinaryOperator::Subtract)
                    | (Type::Number, Type::Number, BinaryOperator::Multiply)
                    | (Type::Number, Type::Number, BinaryOperator::Divide)
                    | (Type::Number, Type::Number, BinaryOperator::Modulo) => {
                        let left_float = left_val.into_float_value();
                        let right_float = right_val.into_float_value();

                        let result = match &operator {
                            BinaryOperator::Add => self
                                .builder
                                .build_float_add(left_float, right_float, "add")
                                .unwrap(),
                            BinaryOperator::Subtract => self
                                .builder
                                .build_float_sub(left_float, right_float, "sub")
                                .unwrap(),
                            BinaryOperator::Multiply => self
                                .builder
                                .build_float_mul(left_float, right_float, "mul")
                                .unwrap(),
                            BinaryOperator::Divide => self
                                .builder
                                .build_float_div(left_float, right_float, "div")
                                .unwrap(),
                            BinaryOperator::Modulo => self
                                .builder
                                .build_float_rem(left_float, right_float, "rem")
                                .unwrap(),
                            _ => unreachable!(),
                        };
                        Ok(result.into())
                    }
                    (Type::Number, Type::Number, BinaryOperator::Equal)
                    | (Type::Number, Type::Number, BinaryOperator::NotEqual)
                    | (Type::Number, Type::Number, BinaryOperator::StrictEqual)
                    | (Type::Number, Type::Number, BinaryOperator::StrictNotEqual)
                    | (Type::Number, Type::Number, BinaryOperator::LessThan)
                    | (Type::Number, Type::Number, BinaryOperator::LessEqual)
                    | (Type::Number, Type::Number, BinaryOperator::GreaterThan)
                    | (Type::Number, Type::Number, BinaryOperator::GreaterEqual) => {
                        let left_float = left_val.into_float_value();
                        let right_float = right_val.into_float_value();

                        let predicate = match operator {
                            BinaryOperator::Equal => FloatPredicate::OEQ,
                            BinaryOperator::NotEqual => FloatPredicate::ONE,
                            BinaryOperator::StrictEqual => FloatPredicate::OEQ,
                            BinaryOperator::StrictNotEqual => FloatPredicate::ONE,
                            BinaryOperator::LessThan => FloatPredicate::OLT,
                            BinaryOperator::LessEqual => FloatPredicate::OLE,
                            BinaryOperator::GreaterThan => FloatPredicate::OGT,
                            BinaryOperator::GreaterEqual => FloatPredicate::OGE,
                            _ => unreachable!(),
                        };

                        let result = self
                            .builder
                            .build_float_compare(predicate, left_float, right_float, "cmp")
                            .unwrap();
                        Ok(result.into())
                    }
                    (Type::Boolean, Type::Boolean, BinaryOperator::LogicalAnd)
                    | (Type::Boolean, Type::Boolean, BinaryOperator::LogicalOr)
                    | (Type::Boolean, Type::Boolean, BinaryOperator::Equal)
                    | (Type::Boolean, Type::Boolean, BinaryOperator::NotEqual)
                    | (Type::Boolean, Type::Boolean, BinaryOperator::StrictEqual)
                    | (Type::Boolean, Type::Boolean, BinaryOperator::StrictNotEqual) => {
                        let left_bool = left_val.into_int_value();
                        let right_bool = right_val.into_int_value();

                        let result = match &operator {
                            BinaryOperator::LogicalAnd => self
                                .builder
                                .build_and(left_bool, right_bool, "and")
                                .unwrap(),
                            BinaryOperator::LogicalOr => {
                                self.builder.build_or(left_bool, right_bool, "or").unwrap()
                            }
                            BinaryOperator::Equal => self
                                .builder
                                .build_int_compare(IntPredicate::EQ, left_bool, right_bool, "eq")
                                .unwrap(),
                            BinaryOperator::NotEqual => self
                                .builder
                                .build_int_compare(IntPredicate::NE, left_bool, right_bool, "ne")
                                .unwrap(),
                            BinaryOperator::StrictEqual => self
                                .builder
                                .build_int_compare(
                                    IntPredicate::EQ,
                                    left_bool,
                                    right_bool,
                                    "strict_eq",
                                )
                                .unwrap(),
                            BinaryOperator::StrictNotEqual => self
                                .builder
                                .build_int_compare(
                                    IntPredicate::NE,
                                    left_bool,
                                    right_bool,
                                    "strict_ne",
                                )
                                .unwrap(),
                            _ => unreachable!(),
                        };
                        Ok(result.into())
                    }
                    (_, _, BinaryOperator::NullishCoalescing) => {
                        // Nullish coalescing: return left if not null/undefined, otherwise return right
                        // For now, we'll just return the right operand
                        // In a full implementation, we'd check for null/undefined
                        Ok(right_val)
                    }
                    _ => Err(DrafError::codegen_error(format!(
                        "Binary operation {:?} not implemented for types {:?} and {:?}",
                        operator, left_type, right_type
                    ))),
                }
            }

            Expression::Unary {
                operator, operand, ..
            } => {
                let typed_operand = TypedExpression::new(*operand, expr.type_info.clone());
                let operand_val = self.generate_expression(typed_operand)?;

                match expr.type_info {
                    Type::Number => {
                        let float_val = operand_val.into_float_value();
                        let result = match operator {
                            UnaryOperator::Minus => {
                                self.builder.build_float_neg(float_val, "neg").unwrap()
                            }
                            UnaryOperator::Plus => float_val, // No-op for plus
                            _ => {
                                return Err(DrafError::codegen_error(format!(
                                    "Unary operator {:?} not implemented for numbers",
                                    operator
                                )));
                            }
                        };
                        Ok(result.into())
                    }
                    Type::Boolean => {
                        let bool_val = operand_val.into_int_value();
                        let result = match operator {
                            UnaryOperator::LogicalNot => {
                                self.builder.build_not(bool_val, "not").unwrap()
                            }
                            _ => {
                                return Err(DrafError::codegen_error(format!(
                                    "Unary operator {:?} not implemented for booleans",
                                    operator
                                )));
                            }
                        };
                        Ok(result.into())
                    }
                    _ => Err(DrafError::codegen_error(format!(
                        "Unary operations not implemented for type: {}",
                        expr.type_info
                    ))),
                }
            }

            Expression::Assignment { target, value, .. } => {
                let typed_value = TypedExpression::new(*value, expr.type_info.clone());
                let value_expr = self.generate_expression(typed_value)?;

                // For now, only handle simple variable assignment
                if let Expression::Identifier { name, .. } = target.as_ref() {
                    if let Some(var_ptr) = self.variables.get(name) {
                        self.builder.build_store(*var_ptr, value_expr).unwrap();
                        Ok(value_expr)
                    } else {
                        Err(DrafError::codegen_error(format!(
                            "Assignment to undefined variable: {}",
                            name
                        )))
                    }
                } else {
                    Err(DrafError::codegen_error(
                        "Complex assignment targets not yet implemented",
                    ))
                }
            }

            Expression::ConsoleCall {
                method, arguments, ..
            } => {
                self.generate_console_call(&method, arguments)?;
                // Console calls return void, represented as a dummy value
                Ok(self.context.i32_type().const_int(0, false).into())
            }

            Expression::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                // Generate condition
                let typed_condition = TypedExpression::new(*condition, Type::Boolean);
                let condition_val = self.generate_expression(typed_condition)?;
                let condition_bool = condition_val.into_int_value();

                // Get current function
                let current_fn = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                // Create basic blocks
                let then_block = self.context.append_basic_block(current_fn, "cond_then");
                let else_block = self.context.append_basic_block(current_fn, "cond_else");
                let merge_block = self.context.append_basic_block(current_fn, "cond_merge");

                // Build conditional branch
                self.builder
                    .build_conditional_branch(condition_bool, then_block, else_block)
                    .unwrap();

                // Generate then expression
                self.builder.position_at_end(then_block);
                let typed_then = TypedExpression::new(*then_expr, expr.type_info.clone());
                let then_val = self.generate_expression(typed_then)?;
                let then_block_end = self.builder.get_insert_block().unwrap();
                self.builder
                    .build_unconditional_branch(merge_block)
                    .unwrap();

                // Generate else expression
                self.builder.position_at_end(else_block);
                let typed_else = TypedExpression::new(*else_expr, expr.type_info.clone());
                let else_val = self.generate_expression(typed_else)?;
                let else_block_end = self.builder.get_insert_block().unwrap();
                self.builder
                    .build_unconditional_branch(merge_block)
                    .unwrap();

                // Create phi node in merge block
                self.builder.position_at_end(merge_block);
                let llvm_type = self.type_to_llvm_type(&expr.type_info)?;
                let phi = self.builder.build_phi(llvm_type, "cond_result").unwrap();
                phi.add_incoming(&[(&then_val, then_block_end), (&else_val, else_block_end)]);

                Ok(phi.as_basic_value())
            }

            _ => Err(DrafError::codegen_error(
                "Expression type not yet implemented in codegen",
            )),
        }
    }

    /// Convert a Draf type to LLVM type
    fn type_to_llvm_type(&self, ty: &Type) -> DrafResult<BasicTypeEnum<'ctx>> {
        match ty {
            Type::Number => Ok(self.context.f64_type().into()),
            Type::String => Ok(self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()),
            Type::Boolean => Ok(self.context.bool_type().into()),
            Type::Null | Type::Undefined => Ok(self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()),
            Type::Any => {
                // For now, represent Any as a pointer (will need proper tagged union later)
                Ok(self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into())
            }
            Type::Void => Err(DrafError::codegen_error(
                "Void type cannot be used as value type",
            )),
            Type::Never => Err(DrafError::codegen_error(
                "Never type cannot be instantiated",
            )),
            _ => Err(DrafError::codegen_error(format!(
                "Type not yet implemented in codegen: {}",
                ty
            ))),
        }
    }

    /// Generate console call (printf-based)
    fn generate_console_call(
        &mut self,
        method: &ConsoleMethod,
        arguments: Vec<Expression>,
    ) -> DrafResult<()> {
        let printf_fn = self.printf_function.unwrap();

        // Create format string based on console method
        let (prefix, _use_stderr) = match method {
            ConsoleMethod::Log => ("", false),
            ConsoleMethod::Info => ("[INFO] ", false),
            ConsoleMethod::Warn => ("[WARN] ", true),
            ConsoleMethod::Error => ("[ERROR] ", true),
            ConsoleMethod::Debug => ("[DEBUG] ", false),
        };

        // Infer types for arguments and build appropriate format string
        let mut format_str = prefix.to_string();
        let mut printf_args: Vec<BasicMetadataValueEnum> = Vec::new();
        let mut converted_values: Vec<BasicValueEnum> = Vec::new();

        // First pass: generate values and infer types
        for arg_expr in &arguments {
            let inferred_type = self.infer_expression_type(arg_expr)?;
            let typed_arg = TypedExpression::new(arg_expr.clone(), inferred_type.clone());
            let arg_value = self.generate_expression(typed_arg)?;
            converted_values.push(arg_value);
        }

        // Second pass: build format string and convert values as needed
        for (i, (arg_expr, arg_value)) in arguments.iter().zip(converted_values.iter()).enumerate()
        {
            if i > 0 {
                format_str.push(' ');
            }

            let inferred_type = self.infer_expression_type(arg_expr)?;
            match inferred_type {
                Type::Boolean => {
                    // Convert boolean to string representation
                    format_str.push_str("%s");

                    // Create string constants for true/false
                    let bool_str = if arg_value.is_int_value() {
                        let int_val = arg_value.into_int_value();
                        let is_true = self
                            .builder
                            .build_int_compare(
                                inkwell::IntPredicate::NE,
                                int_val,
                                self.context.bool_type().const_zero(),
                                "bool_check",
                            )
                            .unwrap();

                        // Create conditional selection of string
                        let true_str = self.context.const_string(b"true\0", false);
                        let false_str = self.context.const_string(b"false\0", false);

                        let true_global =
                            self.module
                                .add_global(true_str.get_type(), None, "true_str");
                        true_global.set_initializer(&true_str);
                        true_global.set_constant(true);

                        let false_global =
                            self.module
                                .add_global(false_str.get_type(), None, "false_str");
                        false_global.set_initializer(&false_str);
                        false_global.set_constant(true);

                        let true_ptr = self
                            .builder
                            .build_pointer_cast(
                                true_global.as_pointer_value(),
                                self.context.i8_type().ptr_type(AddressSpace::default()),
                                "true_ptr",
                            )
                            .unwrap();

                        let false_ptr = self
                            .builder
                            .build_pointer_cast(
                                false_global.as_pointer_value(),
                                self.context.i8_type().ptr_type(AddressSpace::default()),
                                "false_ptr",
                            )
                            .unwrap();

                        self.builder
                            .build_select(is_true, true_ptr, false_ptr, "bool_str")
                            .unwrap()
                    } else {
                        // Fallback for non-int boolean values
                        let false_str = self.context.const_string(b"false\0", false);
                        let false_global = self.module.add_global(
                            false_str.get_type(),
                            None,
                            "false_str_fallback",
                        );
                        false_global.set_initializer(&false_str);
                        false_global.set_constant(true);

                        self.builder
                            .build_pointer_cast(
                                false_global.as_pointer_value(),
                                self.context.i8_type().ptr_type(AddressSpace::default()),
                                "false_ptr_fallback",
                            )
                            .unwrap()
                            .into()
                    };

                    printf_args.push(bool_str.into());
                }
                Type::Number => {
                    format_str.push_str("%.2f");
                    printf_args.push((*arg_value).into());
                }
                _ => {
                    // Default to number format for other types
                    format_str.push_str("%.2f");
                    printf_args.push((*arg_value).into());
                }
            }
        }

        format_str.push('\n');
        format_str.push('\0'); // Null terminate

        // Create format string constant
        let format_string = self.context.const_string(format_str.as_bytes(), false);
        let global_fmt = self
            .module
            .add_global(format_string.get_type(), None, "fmt_str");
        global_fmt.set_initializer(&format_string);
        global_fmt.set_constant(true);

        // Get pointer to format string
        let fmt_ptr = self
            .builder
            .build_pointer_cast(
                global_fmt.as_pointer_value(),
                self.context.i8_type().ptr_type(AddressSpace::default()),
                "fmt_ptr",
            )
            .unwrap();

        // Prepare final printf arguments
        let mut final_printf_args: Vec<BasicMetadataValueEnum> = vec![fmt_ptr.into()];
        final_printf_args.extend(printf_args);

        // Call printf
        self.builder
            .build_call(printf_fn, &final_printf_args, "printf_call")
            .unwrap();

        Ok(())
    }

    /// Infer the type of an expression for console output
    fn infer_expression_type(&self, expr: &Expression) -> DrafResult<Type> {
        match expr {
            Expression::Literal { value, .. } => Ok(match value {
                LiteralValue::Number(_) => Type::Number,
                LiteralValue::String(_) => Type::String,
                LiteralValue::Boolean(_) => Type::Boolean,
                LiteralValue::Null => Type::Null,
                LiteralValue::Undefined => Type::Undefined,
            }),
            Expression::Identifier { name, .. } => {
                // Look up the variable type from our stored type information
                if let Some(var_type) = self.variable_types.get(name) {
                    Ok(var_type.clone())
                } else {
                    Ok(Type::Number) // Default fallback for unknown variables
                }
            }
            Expression::Binary { operator, .. } => {
                // Infer result type based on operator
                match operator {
                    BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::StrictEqual
                    | BinaryOperator::StrictNotEqual
                    | BinaryOperator::LessThan
                    | BinaryOperator::LessEqual
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::LogicalAnd
                    | BinaryOperator::LogicalOr => Ok(Type::Boolean),
                    BinaryOperator::NullishCoalescing => Ok(Type::Number), // Simplified for now
                    _ => Ok(Type::Number),
                }
            }
            Expression::Unary { operator, .. } => match operator {
                UnaryOperator::LogicalNot => Ok(Type::Boolean),
                _ => Ok(Type::Number),
            },
            Expression::Conditional { then_expr, .. } => {
                // For simplicity, use the type of the then branch
                self.infer_expression_type(then_expr)
            }
            _ => Ok(Type::Number), // Default fallback
        }
    }

    /// Get the generated module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
}

/// Generate LLVM IR from typed AST
pub fn generate(program: TypedProgram, target_triple: Option<&str>) -> DrafResult<String> {
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "draf_program");

    // Set target triple if provided
    if let Some(triple) = target_triple {
        codegen
            .module
            .set_triple(&inkwell::targets::TargetTriple::create(triple));
    }

    codegen.generate(program)?;
    Ok(codegen.module.print_to_string().to_string())
}

/// Write object file from LLVM IR string
pub fn write_object_file<P: AsRef<Path>>(
    ir_code: &str,
    output_path: P,
    optimization_level: u8,
) -> DrafResult<()> {
    let output_path = output_path.as_ref();

    // Write IR to temporary file
    let ir_file = output_path.with_extension("ll");
    std::fs::write(&ir_file, ir_code)
        .map_err(|e| DrafError::io_error(format!("Failed to write IR file: {}", e)))?;

    // Determine optimization flag
    let opt_flag = match optimization_level {
        0 => "-O0",
        1 => "-O1",
        2 => "-O2",
        3 => "-O3",
        _ => "-O2",
    };

    // Use clang to compile IR to object file
    let output = Command::new("clang")
        .arg("-c")
        .arg(opt_flag)
        .arg(&ir_file)
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|e| DrafError::codegen_error(format!("Failed to run clang: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DrafError::codegen_error(format!(
            "Clang compilation failed: {}",
            stderr
        )));
    }

    // Clean up temporary IR file
    let _ = std::fs::remove_file(&ir_file);

    Ok(())
}

/// Generate executable from LLVM IR
pub fn write_executable<P: AsRef<Path>>(
    ir_code: &str,
    output_path: P,
    optimization_level: u8,
) -> DrafResult<()> {
    let output_path = output_path.as_ref();

    // Write IR to temporary file
    let ir_file = output_path.with_extension("ll");
    std::fs::write(&ir_file, ir_code)
        .map_err(|e| DrafError::io_error(format!("Failed to write IR file: {}", e)))?;

    // Determine optimization flag
    let opt_flag = match optimization_level {
        0 => "-O0",
        1 => "-O1",
        2 => "-O2",
        3 => "-O3",
        _ => "-O2",
    };

    // Use clang to compile IR directly to executable
    let output = Command::new("clang")
        .arg(opt_flag)
        .arg(&ir_file)
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|e| DrafError::codegen_error(format!("Failed to run clang: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DrafError::codegen_error(format!(
            "Clang compilation failed: {}",
            stderr
        )));
    }

    // Clean up temporary IR file
    let _ = std::fs::remove_file(&ir_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser;
    use crate::semantic;

    #[test]
    fn test_number_literal_codegen() {
        let source = "let x: number = 42.5;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let ir = result.unwrap();

        // Check that the IR contains our constant
        assert!(ir.contains("42.5"));
    }

    #[test]
    fn test_binary_operation_codegen() {
        let source = "let x = 1.0 + 2.0;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let ir = result.unwrap();

        // Check that the IR contains floating point addition
        assert!(ir.contains("fadd"));
    }

    #[test]
    fn test_variable_assignment_codegen() {
        let source = r#"
            let x: number = 10.0;
            x = 20.0;
        "#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let ir = result.unwrap();

        // Check that the IR contains alloca and store operations
        assert!(ir.contains("alloca"));
        assert!(ir.contains("store"));
    }

    #[test]
    fn test_boolean_operations_codegen() {
        let source = "let x = true && false;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let ir = result.unwrap();

        // Check that the IR contains boolean operations
        assert!(ir.contains("and"));
    }

    #[test]
    fn test_unary_operations_codegen() {
        let source = "let x = -42.0;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let ir = result.unwrap();

        // Check that the IR contains floating point negation
        assert!(ir.contains("fneg"));
    }

    #[test]
    fn test_module_verification() {
        let source = "let x: number = 42;";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let typed_program = semantic::analyze(program).unwrap();

        let result = generate(typed_program, None);
        assert!(result.is_ok());

        let _ir = result.unwrap();
        // Just check that we got IR code successfully
        assert!(true);
    }
}
