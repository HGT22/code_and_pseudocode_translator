# Estructura del Proyecto - Code Translator

## Jerarquía de carpetas

```
Code Translator/
│
├── Cargo.toml                    # Configuración del proyecto Rust
├── Cargo.lock                    # Bloqueo de versiones (generado)
├── README.md                     # Documentación principal
├── INSTALL_RUST.md              # Guía de instalación de Rust
├── EXAMPLES.md                  # Ejemplos de traducción
├── PROJECT_STRUCTURE.md         # Este archivo
├── .gitignore                   # Archivos ignorados por Git
├── .git/                        # Repositorio Git
│
├── src/                          # Código fuente principal
│   ├── main.rs                   # Aplicación CLI (punto de entrada)
│   ├── lib.rs                    # Librería (exporta módulos públicos)
│   ├── commands.rs               # Comandos de la interfaz CLI
│   │
│   ├── core/                     # Núcleo del traductor
│   │   ├── mod.rs               # Agregador de módulos
│   │   │
│   │   ├── lexer/
│   │   │   └── mod.rs           # Análisis léxico
│   │   │       • TokenType enum
│   │   │       • Token struct
│   │   │       • Lexer struct con métodos
│   │   │       • Funciones de tokenización
│   │   │
│   │   ├── parser/
│   │   │   └── mod.rs           # Análisis sintáctico
│   │   │       • Parser struct
│   │   │       • Métodos de análisis recursivo
│   │   │       • Construcción del AST
│   │   │       • Manejo de precedencia
│   │   │
│   │   ├── ast/
│   │   │   └── mod.rs           # Árbol de Sintaxis Abstracta
│   │   │       • ASTNode enum
│   │   │       • DataType enum
│   │   │       • BinaryOperator enum
│   │   │       • UnaryOperator enum
│   │   │       • Parameter struct
│   │   │
│   │   └── transpiler/
│   │       └── mod.rs           # Generador de código
│   │           • CodeGenerator struct
│   │           • Métodos para cada lenguaje
│   │           • Conversión de tipos
│   │           • Generación de sintaxis específica
│   │
│   └── cli/
│       └── mod.rs               # Interfaz de usuario
│           • CodeTranslator struct
│           • Métodos de traducción
│           • Orquestación de componentes
│
└── target/                       # Directorio de compilación (generado)
    ├── debug/                    # Binarios de debug
    └── release/                  # Binarios optimizados
        └── code-translator.exe
```

## Flujo de datos

```
CÓDIGO FUENTE (C, Python, JS, Pseudocode)
         ↓
    LEXER (src/core/lexer)
         ↓
    TOKENS (estructura intermedia)
         ↓
    PARSER (src/core/parser)
         ↓
    AST UNIVERSAL (src/core/ast)
         ↓
    TRANSPILER (src/core/transpiler)
         ↓
CÓDIGO GENERADO (formato de salida)
```

## Componentes clave

### 1. Lexer (Análisis Léxico)
- **Responsabilidad**: Convertir el texto de entrada en tokens
- **Entrada**: String del código fuente
- **Salida**: Vec<Token>
- **Características**:
  - Identifica palabras clave
  - Reconoce operadores y puntuación
  - Maneja strings y números
  - Soporta comentarios

### 2. Parser (Análisis Sintáctico)
- **Responsabilidad**: Construir un AST desde tokens
- **Entrada**: Vec<Token>
- **Salida**: ASTNode
- **Características**:
  - Análisis recursivo descendente
  - Precedencia de operadores
  - Manejo de errores con linea/columna
  - Soporta todas las estructuras de control

### 3. AST (Árbol de Sintaxis Abstracta)
- **Responsabilidad**: Representación universal del código
- **Características**:
  - Independiente del lenguaje de entrada
  - Flexible para múltiples lenguajes de salida
  - Tipos de datos comunes
  - Operadores normalizados

### 4. Transpiler (Generador de Código)
- **Responsabilidad**: Convertir AST a código en lenguaje objetivo
- **Entrada**: ASTNode, Language
- **Salida**: String (código traducido)
- **Características**:
  - Generación específica por lenguaje
  - Indentación automática
  - Conversión de tipos
  - Headers y librerías apropiadas

### 5. CLI (Interfaz de Usuario)
- **Responsabilidad**: Orquestar la traducción
- **Características**:
  - Menú interactivo
  - Entrada de usuario
  - Manejo de errores
  - Visualización de resultados

## Archivos de configuración

### Cargo.toml
- Nombre: `code-translator`
- Versión: `0.1.0`
- Edition: `2021`
- Dependencias: regex, lazy_static, clap, anyhow, thiserror

### .gitignore
- Excluye `/target/` (compilación)
- Excluye binarios generados
- Excluye archivos IDE
- Excluye archivos de entorno

## Dependencias del Proyecto

```toml
[dependencies]
regex = "1.10"              # Expresiones regulares
lazy_static = "1.4"         # Inicialización lenta de estáticos
clap = { version = "4.4" }  # CLI argument parsing (futuro)
anyhow = "1.0"              # Manejo de errores
thiserror = "1.0"           # Derivación de Error trait
```

## Testing

### Estructura de tests
- Tests unitarios en cada módulo
- Tests de integración en lib.rs
- Uso de `#[test]` para casos de prueba

### Ejecutar tests
```bash
cargo test                # Todos los tests
cargo test --lib        # Solo tests de librería
cargo test lexer        # Tests de módulo específico
```

## Compilación Multiplataforma

### Windows (.exe)
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Linux (si es necesario)
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS (si es necesario)
```bash
cargo build --release --target x86_64-apple-darwin
```

## Próximos pasos de desarrollo

1. **Mejorar Parser**
   - Agregar soporte para clases
   - Manejo de templates/genéricos
   - Mejor manejo de errores

2. **Expandir Transpiler**
   - Optimización de código generado
   - Mejora de indentación
   - Conversión de tipos más inteligente

3. **Agregar Lenguajes**
   - Rust
   - Go
   - Java
   - TypeScript

4. **Herramientas**
   - Web UI
   - API REST
   - IDE plugins

---

**Última actualización**: Marzo 2026
**Desarrollado para**: URV - Fundamentos de Programación II
