use std::collections::HashMap;
use crate::ast::expression_struct::Expression;
use crate::ast::function_struct::Function;

/// Environment stores variable bindings during evaluation.
pub struct Environment<'a> {
    pub variables: HashMap<String, Expression>,
    pub functions: HashMap<String, &'a Function>,
}

impl<'a> Environment<'a> {
    pub fn new(functions: HashMap<String, &'a Function>) -> Self {
        Self {
            variables: HashMap::new(),
            functions,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }

    pub fn get_function(&self, name: &str) -> Option<&'a Function> {
        self.functions.get(name).copied()
    }

    pub fn insert_variable(&mut self, name: String, value: Expression) {
        self.variables.insert(name, value);
    }
}