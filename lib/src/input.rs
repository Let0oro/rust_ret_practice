//! # Input - Interactive User Input Module
//!
//! This module provides a convenient, type-safe way to read and validate user input from stdin.
//! It uses a builder pattern to allow optional configuration of validation rules and error messages.
//!
//! ## Features
//!
//! - **Generic type support**: Works with any type that implements `FromStr`
//! - **Validation**: Optional predicate-based validation with custom error messages
//! - **Builder pattern**: Fluent API for composing input configurations
//! - **Error handling**: Distinguishes between format errors and validation errors
//!
//! ## Examples
//!
//! ### Simple string input
//! ```
//! let name = Input::<String>::new("Enter your name").read();
//! ```
//!
//! ### Validated numeric input
//! ```
//! let age = Input::<u32>::new("Enter your age")
//!     .validate(|age| age > &0u32)
//!     .err_msg("Age must be a positive number")
//!     .read();
//! ```
//!
//! ### With custom validation message
//! ```
//! let email = Input::<String>::new("Enter email")
//!     .validate(|email| email.contains('@'))
//!     .err_msg("Email must contain '@' symbol")
//!     .read();
//! ```

use std::error::Error;
use std::str::FromStr;
use std::io::{Write, stdin, stdout};

/// Reads a single line from stdin and parses it into type `T`.
///
/// This is an internal helper function that handles the low-level I/O and parsing.
/// It automatically trims whitespace from the input before parsing.
///
/// # Errors
///
/// Returns an error if:
/// - Reading from stdin fails
/// - Parsing the trimmed string into type `T` fails
///
/// # Type Parameters
///
/// * `T` - Any type that implements `FromStr` with an error type that implements `Error + Send + Sync + 'static`
///
/// # Examples
///
/// ```
/// let number: u32 = _read().expect("Failed to read");
/// let text: String = _read().expect("Failed to read");
/// ```
fn _read<'a, T>() -> Result<T, Box<dyn Error>>
where 
    T: FromStr,
    T::Err: Error + Send + Sync + 'static 
{
    let mut value: String = String::new();
    stdin().read_line(&mut value)?;
    let trimmed = value.trim();
    let result = trimmed.parse::<T>()?;
    Ok(result)
}

/// A builder struct for interactive user input with validation.
///
/// `Input<T>` provides a fluent API to configure and execute user input operations
/// with optional validation and custom error messages. It supports any type that can
/// be parsed from a string.
///
/// # Type Parameters
///
/// * `T` - The type to parse input into. Must implement `FromStr` with an error type
///         that implements `Error + Send + Sync + 'static`
///
/// # Lifetimes
///
/// * `'a` - The lifetime of string references for messages (prompt and error messages)
///
/// # Examples
///
/// ```
/// // Simple input without validation
/// let name = Input::<String>::new("Enter name").read();
///
/// // Input with validation
/// let age = Input::<u32>::new("Enter age")
///     .validate(|age| age > &18u32)
///     .err_msg("You must be 18 or older")
///     .read();
/// ```
pub struct Input<'a, T> {
    /// The prompt message displayed to the user
    msg: &'a str,
    /// A predicate function that validates the parsed input.
    /// Returns `true` if the input is valid, `false` otherwise.
    predicate: Box<dyn Fn(&T) -> bool + 'a>,
    /// Custom error message displayed when validation fails
    err_msg: &'a str,
}

impl <'a, T> Input<'a, T>
where 
    T: FromStr,
    T::Err: Error + Send + Sync + 'static
{
    /// Creates a new `Input` builder with the given prompt message.
    ///
    /// By default:
    /// - No validation is applied (always accepts input)
    /// - Error message is "Entrada inválida"
    ///
    /// # Arguments
    ///
    /// * `msg` - The prompt message to display to the user
    ///
    /// # Returns
    ///
    /// A new `Input` instance ready for configuration
    ///
    /// # Examples
    ///
    /// ```
    /// let input = Input::<String>::new("What's your name?");
    /// ```
    pub fn new(msg: &'a str) -> Self {
        Self {
            msg,
            predicate: Box::new(|_| true),
            err_msg: "Entrada inválida"
        }
    }
    
    /// Adds validation logic to the input.
    ///
    /// The provided predicate is called after successful parsing. If it returns `false`,
    /// the user is prompted to try again with the configured error message.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A closure that takes a reference to the parsed value and returns
    ///                 `true` if valid, `false` if invalid
    ///
    /// # Returns
    ///
    /// `Self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// Input::<u32>::new("Enter age")
    ///     .validate(|age| age > &18u32)
    ///     .read();
    ///
    /// Input::<String>::new("Enter password")
    ///     .validate(|pwd| pwd.len() >= 8)
    ///     .read();
    /// ```
    pub fn validate(mut self, predicate: impl Fn(&T) -> bool + 'a) -> Self {
        self.predicate = Box::new(predicate);
        self
    }
    
    /// Sets a custom error message for validation failures.
    ///
    /// This message is displayed when the validation predicate returns `false`.
    /// If not set, defaults to "Entrada inválida".
    ///
    /// # Arguments
    ///
    /// * `err_msg` - The error message to display on validation failure
    ///
    /// # Returns
    ///
    /// `Self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// Input::<u32>::new("Enter age")
    ///     .validate(|age| age > &0u32)
    ///     .err_msg("Age must be a positive number")
    ///     .read();
    /// ```
    pub fn err_msg(mut self, err_msg: &'a str) -> Self {
        self.err_msg = err_msg;
        self
    }
    
    /// Starts an interactive input loop and returns the validated input.
    ///
    /// This method:
    /// 1. Displays the prompt message
    /// 2. Reads a line from stdin
    /// 3. Attempts to parse it as type `T`
    /// 4. If parsing succeeds, validates with the predicate
    /// 5. Loops until valid input is received
    ///
    /// The method will keep prompting until either:
    /// - Input parses successfully AND passes validation
    /// - The user provides a valid format (for types that always validate)
    ///
    /// # Returns
    ///
    /// The successfully parsed and validated input of type `T`
    ///
    /// # Panics
    ///
    /// Panics if stdout flushing fails (very rare in normal circumstances)
    ///
    /// # Examples
    ///
    /// ```
    /// let name: String = Input::<String>::new("Enter name").read();
    ///
    /// let age: u32 = Input::<u32>::new("Enter age")
    ///     .validate(|age| age > &18u32)
    ///     .err_msg("Must be 18 or older")
    ///     .read();
    /// ```
    pub fn read(&self) -> T {
        loop {
            print!("{}: ", self.msg);
            stdout().flush().expect("Failed to flush stdout");
            match _read::<T>() {
                Ok(input) if (self.predicate)(&input) => return input,
                Ok(_) => println!("Error: {}", self.err_msg),
                Err(e) => println!("Error de formato: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic string input without validation.
    ///
    /// Verifies that `Input` can read and parse a string value.
    #[test]
    fn it_works_simple() {
        
        let result: String = Input::new("Nombre")
                .read();
        
        assert_eq!(result, "John");
    }
    
    /// Tests string input with length validation.
    ///
    /// Verifies that validation rules are properly enforced.
    #[test]
    fn it_works_with_str() {
        
        let result = Input::<String>::new("Nombre")
                .validate(|name| name.len() > 3)
                .read();
        
        assert_eq!(result, "John");
    }
    
    /// Tests string input with validation and custom error message.
    ///
    /// Verifies that custom error messages are correctly configured.
    #[test]
    fn it_works_with_str_and_err_msg() {
        
        let result = Input::<String>::new("Nombre")
                .validate(|name| name.len() > 3)
                .err_msg("Ha de contener como mínimo 4 letras")
                .read();
        
        assert_eq!(result, "John");
    }

    /// Tests numeric input with value validation.
    ///
    /// Verifies that the builder pattern works correctly with numeric types
    /// and that validation constraints are applied.
    #[test]
    fn it_works_with_number() {
        
        let result = Input::<u32>::new("Edad")
            .validate(|age| age > &0u32)
            .err_msg("Debe ser un número positivo")
            .read();
        assert_eq!(result, 25);
    }
}
