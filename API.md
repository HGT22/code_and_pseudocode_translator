# API Interna del Code Translator

## Módulos Públicos

### Language Enum
```rust
pub enum Language {
    C,
    Python,
    JavaScript,
    Pseudocode,
}
```

**Métodos**:
- `from_string(s: &str) -> Option<Self>` - Convierte string a Language
- `to_string(&self) -> &'static str` - Convierte a representación string
- `supported_languages() -> Vec<&'static str>` - Lista de lenguajes soportados

---

## Módulo Core

### core::lexer

```rust
pub struct Lexer {
    pub fn new(input: &str) -> Self
    pub fn tokenize(&mut self) -> Vec<Token>
}

pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub enum TokenType {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    // ... más tipos
}
```

---

### core::parser

```rust
pub struct Parser {
    pub fn new(tokens: Vec<Token>) -> Self
    pub fn parse(&mut self) -> Result<ASTNode, String>
}

pub fn parse(input: &str) -> Result<ASTNode, String>
```

---

### core::ast

```rust
pub enum ASTNode {
    Program(Vec<Box<ASTNode>>),
    VarDeclaration { name: String, data_type: DataType, value: Option<Box<ASTNode>> },
    FunctionDef { name: String, params: Vec<Parameter>, return_type: DataType, body: Box<ASTNode> },
    IfStatement { condition: Box<ASTNode>, then_branch: Box<ASTNode>, else_branch: Option<Box<ASTNode>> },
    // ... más nodos
}

pub enum DataType {
    Integer, Float, String, Boolean, Void, Array, Any,
}

pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide, Modulo,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    And, Or, BitwiseAnd, BitwiseOr, BitwiseXor,
}

pub enum UnaryOperator {
    Not, Negate, BitwiseNot,
}

pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}
```

---

### core::transpiler

```rust
pub struct CodeGenerator {
    pub fn new(language: Language) -> Self
    pub fn generate(&self, ast: &ASTNode) -> String
}

pub fn generate_code(ast: &ASTNode, target_language: Language) -> String
```

---

## Módulo CLI

### cli::CodeTranslator

```rust
pub struct CodeTranslator;

impl CodeTranslator {
    pub fn new() -> Self
    
    pub fn translate(
        &self, 
        code: &str, 
        source_lang: Language, 
        target_lang: Language
    ) -> Result<String, String>
    
    pub fn translate_file(
        &self, 
        file_path: &str, 
        source_lang: Language, 
        target_lang: Language
    ) -> Result<String, String>
}
```

---

## Ejemplos de Uso

### Traducción Simple
```rust
use code_translator::{Language, cli::CodeTranslator};

fn main() {
    let translator = CodeTranslator::new();
    
    let code = "let x = 10;";
    let result = translator.translate(
        code,
        Language::Python,
        Language::C
    );
    
    match result {
        Ok(translated) => println!("{}", translated),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Procesamiento de AST
```rust
use code_translator::core::{ lexer::Lexer, parser::Parser };

fn main() {
    let code = "let x = 10;";
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    println!("{:?}", ast);
}
```

### Generación de Código
```rust
use code_translator::{ Language, core::transpiler::CodeGenerator };

fn main() {
    let ast = /* AST obtenido del parser */;
    
    let generator = CodeGenerator::new(Language::C);
    let code = generator.generate(&ast);
    
    println!("Código C:\n{}", code);
}
```

---

## Códigos de Error

### Parser Errors
- `"Expected identifier, got ..."`
- `"Could not parse type"`
- `"Unexpected token: ..."`
- `"Unclosed bracket"`
- `"Invalid syntax"`

### Lexer Errors
- `"Invalid token"`
- `"Unclosed string"`
- `"Invalid number format"`

### Transpiler Errors
- `"Unknown data type"`
- `"Invalid operation"`

---

## Extensión del Traductor

### Agregar un nuevo lenguaje

1. **Extensión del enum Language**
```rust
pub enum Language {
    // ...
    Rust,
}

impl Language {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            // ...
            "rust" => Some(Language::Rust),
            _ => None,
        }
    }
}
```

2. **Agregar generador en transpiler**
```rust
fn generate_node(&self, node: &ASTNode) -> String {
    match self.language {
        // ...
        Language::Rust => self.generate_rust(node),
    }
}

fn generate_rust(&self, node: &ASTNode) -> String {
    // Implementación específica de Rust
}
```

3. **Actualizar lexer si es necesario**
```rust
// Agregar palabras clave de Rust si es necesario
"fn" => TokenType::Function,
"let" => TokenType::Let,
```

---

## Tests

### Ejecutar tests
```bash
cargo test                          # Todos
cargo test -- --nocapture         # Con output
cargo test lexer                   # Módulo específico
cargo test test_tokenize           # Test específico
```

### Estructura de tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "let x = 10;";
        
        // Act
        let result = process(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

---

## Performance

### Optimizaciones actuales
- Stack-based parsing (sin recursion limit)
- Single-pass lexing
- Direct AST generation

### Posibles mejoras
- Caching de tokens
- Lazy evaluation
- Parallel processing

---

## Información de Compilación

**Versión**: 0.1.0
**Edición Rust**: 2021
**MSRV** (Minimum Supported Rust Version): 1.70

**Dependencias principales**:
- `regex` - Procesamiento de expresiones regulares
- `lazy_static` - Inicialización estática lazy
- `anyhow` - Manejo ergonómico de errores
- `thiserror` - Marcos para tipos de error

---

**Última actualización**: Marzo 2026
**Desarrollado por**: URV - Fundamentos de Programación II
