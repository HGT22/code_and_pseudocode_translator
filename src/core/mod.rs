// Core module - Contains all translation components
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod transpiler;
pub use transpiler::CodeGenerator;
