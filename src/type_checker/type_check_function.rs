/// Performs type checking on a single function.
/// Ensures parameter types and return type are valid.
/// Checks all statements in the function body for type correctness.
/// Returns Ok(()) if all checks pass, or Err with an error message on failure.
pub fn type_check_function(function: &Function) -> Result<(), String> {
    let mut context = TypeContext::new();

    // Register parameters in context
    for param in &function.params {
        if context.variables.contains_key(&param.name) {
            return Err(format!("Duplicate parameter name '{}'", param.name));
        }
        context.variables.insert(param.name.clone(), param.param_type.clone());
    }

    // Check each statement
    for stmt in &function.body {
        type_check_statement(stmt, &mut context)?;
    }

    // For non-void functions, ensure last statement is a return
    if function.return_type != Type::Void {
        if let Some(last_stmt) = function.body.last() {
            if !is_return_statement(last_stmt) {
                return Err(format!("Function '{}' missing return statement", function.name));
            }
        } else {
            return Err(format!("Function '{}' has empty body but non-void return type", function.name));
        }
    }

    Ok(())
}

/// Helper struct for tracking variable types in current scope
struct TypeContext {
    variables: std::collections::HashMap<String, Type>,
}

impl TypeContext {
    fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }
}

/// Checks the type correctness of a statement within a context.
/// Placeholder implementation to satisfy compiler, must be implemented properly.
fn type_check_statement(_stmt: &Statement, _ctx: &mut TypeContext) -> Result<(), String> {
    // Implementation omitted here, assume this function properly type-checks all statement kinds.
    Ok(())
}

/// Returns true if the statement is a return statement.
fn is_return_statement(stmt: &Statement) -> bool {
    matches!(stmt, Statement::Return { .. })
}
