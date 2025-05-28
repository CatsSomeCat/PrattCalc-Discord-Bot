//! Execution state management for script termination.
//! 
//! This module manages state for early termination of scripts.

use std::cell::RefCell;

/// Represents the state when an end statement is executed.
///
/// This structure tracks whether a script has terminated early via an `end` statement
/// and the optional return value provided by that statement.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExitState {
    /// Whether an exit has occurred.
    pub occurred: bool,
    
    /// The optional value returned by the exit statement.
    pub value: Option<f32>,
}

impl ExitState {
    /// Creates a new exit state with the given value.
    pub fn with_value(value: Option<f32>) -> Self {
        Self {
            occurred: true,
            value,
        }
    }
    
    /// Resets the exit state to its default.
    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

// Thread-local storage for the exit state
thread_local! {
    static EXIT_STATE: RefCell<ExitState> = RefCell::new(ExitState::default());
}

/// Provides access to the current exit state for the executing script.
///
/// This function allows controlled access to the thread-local exit state,
/// enabling operations like checking if an exit occurred or setting exit values.
pub fn with_exit_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut ExitState) -> R,
{
    EXIT_STATE.with(|cell| f(&mut *cell.borrow_mut()))
} 
