use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use crate::core::error_types::{EvalError, SymbolError, ControlFlowError};
use crate::core::ast_statement::Statement;

/// Stores global constants that are always available to expressions.
///
/// These constants cannot be modified or cleared.
///
/// This provides a layer of immutable, always accessible mathematical constants that persist
/// across expression evaluations. 
///
/// These values are available even when a user clears their context.
pub struct GlobalConstants {
    values: HashMap<String, f32>,
}

impl GlobalConstants {
    /// Creates a new instance with predefined mathematical constants.
    ///
    /// Initializes a set of common mathematical constants like PI, E, etc.
    ///
    /// These constants will be available to all expressions, regardless of context.
    pub fn new() -> Self {
        let mut values = HashMap::new();
        
        // Add common mathematical constants
        values.insert("PI".to_string(), std::f32::consts::PI);
        values.insert("TAU".to_string(), std::f32::consts::PI * 2.0);
        values.insert("E".to_string(), std::f32::consts::E);
         // The golden ratio number
        values.insert("PHI".to_string(), 1.618033988749895);
        values.insert("SQRT2".to_string(), std::f32::consts::SQRT_2);
        values.insert("INFINITY".to_string(), f32::INFINITY);
        
        Self { values }
    }
    
    /// Gets a constant value by name.
    ///
    /// Returns the value of a global constant if it exists, or None otherwise.
    pub fn get(&self, name: &str) -> Option<f32> {
        self.values.get(name).copied()
    }
    
    /// Checks if a name is a global constant.
    ///
    /// Returns true if the given name is a recognized global constant.
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

// Create a singleton instance of GlobalConstants using lazy_static
// This ensures the constants are only initialized once and are available
// throughout the program's lifetime without being recreated
lazy_static! {
    static ref GLOBAL_CONSTANTS: GlobalConstants = GlobalConstants::new();
}

/// Gets a reference to the global constants.
///
/// This function provides access to the singleton GlobalConstants instance.
///
/// Used by the expression evaluator to look up constant values.
pub fn global_constants() -> &'static GlobalConstants {
    &GLOBAL_CONSTANTS
}

/// Stores variables and their values during evaluation.
/// 
/// Also tracks which variables are constants that cannot be modified.
///
/// Provides safe access and modification methods for variables.
///
/// A symbol table for storing variables and constants.
#[derive(Clone, Default)]
pub struct SymbolTable<T: Clone + PartialEq> {
    /// The values of variables and constants.
    pub values: HashMap<String, T>,
    
    /// Names of symbols that are constants and cannot be modified.
    pub constants: HashSet<String>,

    /// Functions defined in this scope.
    pub functions: HashMap<String, (Vec<String>, Statement)>,
    
    /// Procedures defined in this scope.
    pub procedures: HashMap<String, (Vec<String>, Statement)>,
}

impl<T: Clone + PartialEq> SymbolTable<T> {
    /// Creates a new, empty symbol table.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            constants: HashSet::new(),
            functions: HashMap::new(),
            procedures: HashMap::new(),
        }
    }
    
    /// Checks if a symbol is defined (either as a variable or constant).
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    
    /// Gets the value of a symbol.
    pub fn get(&self, name: &str) -> Option<&T> {
        self.values.get(name)
    }
    
    /// Checks if a symbol is a constant.
    pub fn is_constant(&self, name: &str) -> bool {
        self.constants.contains(name)
    }
    
    /// Adds or updates a variable. Returns anerror if trying to modify a constant.
    ///
    /// This method first checks if the name conflicts with a global constant,
    /// then if it's a local constant, before allowing the modification.
    pub fn set_variable(&mut self, name: String, value: T) -> Result<(), EvalError> {
        // First check if it's a global constant
        if global_constants().contains(&name) {
            return Err(SymbolError::ImmutableConstant(name).into());
        }
        
        // Then check if it's a local constant
        if self.is_constant(&name) {
            // Allow the operation if setting to the same value
            if let Some(current_value) = self.values.get(&name) {
                if current_value == &value {
                    // Value is unchanged, allow the operation even on constants
                    return Ok(());
                }
            }
            return Err(SymbolError::ImmutableConstant(name).into());
        }
        self.values.insert(name, value);
        Ok(())
    }
    
    /// Declares a new constant; the constant cannot be modified after declaration.
    ///
    /// Returns anerror if the symbol already exists.
    ///
    /// This method also checks for conflicts with global constants.
    pub fn declare_constant(&mut self, name: String, value: T) -> Result<(), EvalError> {
        // First check if it's a global constant
        if global_constants().contains(&name) {
            return Err(SymbolError::ImmutableConstant(name).into());
        }
        
        // Then check if it exists locally
        if self.values.contains_key(&name) {
            return Err(SymbolError::ImmutableConstant(name).into());
        }
        self.values.insert(name.clone(), value);
        self.constants.insert(name);
        Ok(())
    }
    
    /// Declares a new function with the given name, parameters, and body.
    pub fn declare_function(&mut self, name: String, params: Vec<String>, body: Statement) -> Result<(), EvalError> {
        if self.functions.contains_key(&name) {
            return Err(ControlFlowError::FunctionOrProcedureAlreadyDefined {
                name,
                kind: "Function".to_string(),
            }.into());
        }
        self.functions.insert(name, (params, body));
        Ok(())
    }
    
    /// Declares a new procedure with the given name, parameters, and body.
    pub fn declare_procedure(&mut self, name: String, params: Vec<String>, body: Statement) -> Result<(), EvalError> {
        if self.procedures.contains_key(&name) {
            return Err(ControlFlowError::FunctionOrProcedureAlreadyDefined {
                name,
                kind: "Procedure".to_string(),
            }.into());
        }
        self.procedures.insert(name, (params, body));
        Ok(())
    }
    
    /// Gets a function by name.
    pub fn get_function(&self, name: &str) -> Option<(Vec<String>, Statement)> {
        self.functions.get(name).cloned()
    }
    
    /// Gets a procedure by name.
    pub fn get_procedure(&self, name: &str) -> Option<(Vec<String>, Statement)> {
        self.procedures.get(name).cloned()
    }
    
    /// Creates a new symbol table with the same constants but independent variables.
    ///
    /// Used for creating nested scopes in blocks like if/while statements.
    pub fn new_scope(&self) -> Self {
        Self {
            values: self.values.clone(),
            constants: self.constants.clone(),
            functions: self.functions.clone(),
            procedures: self.procedures.clone(),
        }
    }
    
    /// Merges variables from another scope back into this one.
    ///
    /// Only updates variables that already exist in the outer scope.
    ///
    /// Respects immutability of constants.
    /// 
    /// Used when exiting a scope to propagate changes back to the parent scope.
    #[allow(dead_code)]
    pub fn merge_from_scope(&mut self, other: &Self) -> Result<(), EvalError> {
        for (key, value) in other.values.iter() {
            // Only update variables that already exist in the outer scope
            if !self.contains(key) {
                continue;
            }
            
            // Skip variables that haven't changed
            if self.get(key) == Some(value) {
                continue;
            }
            
            // Don't modify constants from the parent scope
            if self.is_constant(key) {
                continue;
            }
            
            self.set_variable(key.clone(), value.clone())?;
        }
        Ok(())
    }

    /// Returns the number of variables and constants in the symbol table.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Returns true if the symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns true if we're inside a function or procedure context.
    pub fn is_in_callable(&self) -> bool {
        // This is a simple placeholder implementation
        // In a real implementation, you would track the current execution context
        false
    }
}

impl<T: Clone + PartialEq> IntoIterator for SymbolTable<T> {
    type Item = (String, T);
    type IntoIter = std::collections::hash_map::IntoIter<String, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<T: Clone + PartialEq> SymbolTable<T> {
    /// Checks if a variable has the same value.
    #[allow(dead_code)]
    pub fn value_equals(&self, name: &str, value: T) -> bool {
        self.get(name).map_or(false, |v| v == &value)
    }
} 
