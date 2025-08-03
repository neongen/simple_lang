/// Evaluates a single statement within the given mutable environment.
/// Returns `Ok(Some(Expression))` if the statement produces a return value,
/// `Ok(None)` if it does not, or an `Err` with a message on failure.
pub fn evaluate_statement(
    statement: &Statement,
    env: &mut Environment,
) -> Result<Option<Expression>, String> {
    match statement {
        Statement::VariableDeclaration { name, var_type, value } => {
            let val = evaluate_expression(value, env)?;
            if !type_matches(&val, var_type) {
                return Err(format!(
                    "Type mismatch: variable '{}' declared as {:?}, but value has different type",
                    name, var_type
                ));
            }
            env.insert(name.clone(), val);
            Ok(None)
        }
        Statement::FunctionCall { name, args } => {
            let evaluated_args = args
                .iter()
                .map(|arg| evaluate_expression(arg, env))
                .collect::<Result<Vec<_>, _>>()?;
            // Assuming `env.call_function` is a method that executes function by name
            env.call_function(name, evaluated_args)?;
            Ok(None)
        }
        Statement::If { condition, body } => {
            let cond_value = evaluate_expression(condition, env)?;
            if is_truthy(&cond_value) {
                for stmt in body {
                    if let Some(ret) = evaluate_statement(stmt, env)? {
                        return Ok(Some(ret));
                    }
                }
            }
            Ok(None)
        }
        Statement::Return { value } => {
            let ret_val = evaluate_expression(value, env)?;
            Ok(Some(ret_val))
        }
    }
}

/// Checks if an expression value matches the expected type.
fn type_matches(expr: &Expression, expected: &Type) -> bool {
    match (expr, expected) {
        (Expression::IntegerLiteral(_), Type::I32) => true,
        (Expression::StringLiteral(_), Type::String) => true,
        (Expression::FunctionCall { .. }, Type::Void) => true, // Function calls returning void are allowed here
        _ => false,
    }
}

/// Determines if an expression is truthy (non-zero for integers).
fn is_truthy(expr: &Expression) -> bool {
    match expr {
        Expression::IntegerLiteral(i) => *i != 0,
        _ => false, // For simplicity, only integers considered truthy
    }
}

// Dummy stub signatures to allow this code to compile standalone.

/// Environment stores variables and function bindings.
pub struct Environment {
    vars: std::collections::HashMap<String, Expression>,
    // functions: ...
}

impl Environment {
    /// Inserts or updates a variable binding.
    pub fn insert(&mut self, name: String, value: Expression) {
        self.vars.insert(name, value);
    }

    /// Calls a function by name with evaluated arguments.
    /// Returns an error if the function is unknown or call fails.
    pub fn call_function(&mut self, _name: &str, _args: Vec<Expression>) -> Result<(), String> {
        // Stub implementation.
        Ok(())
    }
}

/// Evaluates an expression within the environment.
/// Stub to complete this example.
pub fn evaluate_expression(expr: &Expression, env: &Environment) -> Result<Expression, String> {
    // Stub: in practice, this evaluates recursively and uses env.
    Ok(expr.clone())
}

// Reuse the Expression and Statement definitions for completeness.

#[derive(Clone)]
pub enum Expression {
    IntegerLiteral(i32),
    StringLiteral(String),
    VariableRef(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

pub enum Statement {
    VariableDeclaration {
        name: String,
        var_type: Type,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
    },
    Return {
        value: Expression,
    },
}

#[derive(Debug)]
pub enum Type {
    I32,
    String,
    Void,
}

#[derive(Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
}
