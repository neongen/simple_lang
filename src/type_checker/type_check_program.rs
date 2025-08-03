/// Type-checks the entire program by verifying each function is correctly typed.
/// Returns Ok(()) if all functions pass type checking, otherwise returns an error
/// describing the first encountered type error.
pub fn type_check_program(program: &Program) -> Result<(), String> {
    for function in &program.functions {
        type_check_function(function)?
    }
    Ok(())
}

/// Helper function to type-check a single function.
fn type_check_function(function: &Function) -> Result<(), String> {
    // Create a new context for parameter types
    let mut context = TypeContext::new();

    for param in &function.params {
        if context.contains(&param.name) {
            return Err(format!("Parameter '{}' declared multiple times", param.name));
        }
        context.insert(param.name.clone(), param.param_type.clone());
    }

    for stmt in &function.body {
        type_check_statement(stmt, &context)?
    }

    // TODO: Optionally verify the function's return paths conform to function.return_type

    Ok(())
}

/// Helper function to type-check a statement within a given type context.
fn type_check_statement(stmt: &Statement, context: &TypeContext) -> Result<(), String> {
    match stmt {
        Statement::VariableDeclaration { name, var_type, value } => {
            let expr_type = type_check_expression(value, context)?;
            if &expr_type != var_type {
                return Err(format!(
                    "Type mismatch for variable '{}': expected {:?}, found {:?}",
                    name, var_type, expr_type
                ));
            }
            if context.contains(name) {
                return Err(format!("Variable '{}' redeclared in the same scope", name));
            }
            // Note: Since context is immutable, variable scoping and insertion
            // would require a mutable or layered context in a full impl.
            Ok(())
        }
        Statement::FunctionCall { name: _, args } => {
            for arg in args {
                type_check_expression(arg, context)?;
            }
            Ok(())
        }
        Statement::If { condition, body } => {
            let cond_type = type_check_expression(condition, context)?;
            if cond_type != Type::I32 {
                return Err(format!(
                    "If condition must be of type i32 (interpreted as boolean), found {:?}",
                    cond_type
                ));
            }
            for stmt in body {
                type_check_statement(stmt, context)?;
            }
            Ok(())
        }
        Statement::Return { value } => {
            let _ = type_check_expression(value, context)?;
            Ok(())
        }
    }
}

/// Helper function to type-check an expression within a given type context.
fn type_check_expression(expr: &Expression, context: &TypeContext) -> Result<Type, String> {
    match expr {
        Expression::IntegerLiteral(_) => Ok(Type::I32),
        Expression::StringLiteral(_) => Ok(Type::String),
        Expression::VariableRef(name) => {
            context.get(name).cloned().ok_or_else(|| {
                format!("Use of undeclared variable '{}'", name)
            })
        }
        Expression::BinaryOp { op: _, left, right } => {
            let left_type = type_check_expression(left, context)?;
            let right_type = type_check_expression(right, context)?;
            if left_type != Type::I32 || right_type != Type::I32 {
                return Err("Binary operations require both operands to be i32".to_string());
            }
            Ok(Type::I32)
        }
        Expression::FunctionCall { name: _, args } => {
            // Without full function signature info, assume args are correctly typed.
            for arg in args {
                let _ = type_check_expression(arg, context)?;
            }
            // In a full implementation, would look up function return type.
            Ok(Type::I32) // Placeholder
        }
    }
}

/// Simple type context for name-to-type mapping.
/// Immutable in this minimal example, so using std::collections::HashMap internally.
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeContext(HashMap<String, Type>);

impl TypeContext {
    pub fn new() -> Self {
        TypeContext(HashMap::new())
    }

    pub fn insert(&mut self, name: String, ty: Type) {
        self.0.insert(name, ty);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.0.get(name)
    }
}
