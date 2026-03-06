# Code Translator 🚀

Un traductor modular de lenguajes de programación desarrollado en **Rust** que soporta traducción bidireccional entre múltiples lenguajes. **Multiplataforma**: Windows, macOS, iOS, Android, Linux, ChromeOS y más.

## ✨ Características Principales

- ✅ **Lexer**: Tokenización robusta de código fuente
- ✅ **Parser**: Construcción de Árbol de Sintaxis Abstracta (AST)
- ✅ **AST Universal**: Representación intermedia independiente del lenguaje
- ✅ **Code Generator**: Generación de código en lenguajes objetivo
- ✅ **Interfaz CLI**: Aplicación interactiva en línea de comandos
- ✅ **Multiplataforma**: Windows, macOS, Linux, iOS, Android, ChromeOS
- ✅ **Arquitectura modular**: Fácil de extender y mantener

## 🌍 Plataformas Soportadas

| Plataforma | Estado | Métodos |
|-----------|--------|---------|
| 🪟 **Windows** | ✅ Completo | Ejecutable .exe |
| 🍎 **macOS** | ✅ Completo | App Bundle, DMG |
| 🐧 **Linux** | ✅ Completo | Ejecutable, .deb, .rpm |
| 📱 **iOS** | ✅ Completo | App Store, TestFlight |
| 🤖 **Android** | ✅ Completo | Google Play, APK |
| 🌐 **ChromeOS** | ✅ Completo | Web App, Crostini, Play Store |
| 📺 **tvOS** | ⚠️ Beta | App Store |
| ⌚ **watchOS** | ⚠️ Beta | App Store |

Ver [PLATFORMS.md](PLATFORMS.md) para más detalles sobre cada plataforma.

## 📚 Lenguajes de Programación Soportados

### Entrada/Salida (Bidireccional)
- **C** - Lenguaje compilado
- **C++**, **C#**, **Objective-C**, **Objective-C++**
- **Python**, **Java**, **JavaScript**, **TypeScript**, **Rust**, **Swift**, **Ruby**
- **Go/Golang**, **Kotlin**, **PHP**, **R**, **Scala**, **Dart**, **Haskell**, **Elixir**, **F#**
- **SQL**, **MATLAB**, **D**, **Assembly (x86/x64/ARM)**, **WebAssembly (WAT)**
- **Pseudocódigo orientado a C**, **Pseudocódigo orientado a Java**, **Pseudocódigo orientado a Python**

## Arquitectura

```
src/
├── main.rs              # Punto de entrada CLI
├── lib.rs               # Librería principal
├── commands.rs          # Comandos CLI
├── core/
│   ├── mod.rs          # Agregador de módulos
│   ├── lexer/          # Análisis léxico
│   ├── parser/         # Análisis sintáctico
│   ├── ast/            # Árbol de sintaxis abstracta
│   └── transpiler/     # Generador de código
└── cli/
    └── mod.rs          # Interfaz de usuario
```

## 🛠️ Compilación Rápida

### Windows
```powershell
# Script automático
.\compile-multiplatform.ps1

# O manual
cargo build --release
.\target\release\code-translator.exe
```

### macOS / Linux
```bash
# Compilación automática para tu plataforma
cargo build --release
./target/release/code-translator

# O para compilación cruzada
cargo build --release --target aarch64-apple-darwin  # macOS ARM64
cargo build --release --target x86_64-unknown-linux-gnu  # Linux
```

### iOS / Android
```bash
# iOS
cargo build --release --target aarch64-apple-ios

# Android
cargo install cargo-ndk
cargo ndk -t arm64-v8a build --release

# Ver MULTIPLATFORM_BUILD.md para instrucciones completas
```

### ChromeOS
```bash
# Opción 1: Mediante Crostini (Linux)
# Usa las instrucciones de Linux

# Opción 2: Web App (WASM)
cargo install wasm-pack
wasm-pack build --target web --release
```

## Uso

### Modo Interactivo
```bash
cargo run
```

O ejecuta directamente:
```bash
./code-translator
```

### Flujo de Uso
1. Selecciona "Traducir código"
2. Ingresa el lenguaje de entrada (cualquiera de la lista soportada)
3. Ingresa el lenguaje de salida
4. Pega tu código
5. Escribe "FIN" para terminar
6. ¡Obtén el código traducido!

## Ejemplos de Traducción

### Python → C

**Entrada:**
```python
def factorial(n):
    if n == 1:
        return 1
    return n * factorial(n - 1)
```

**Salida esperada:**
```c
#include <stdio.h>

int factorial(int n) {
    if (n == 1) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

## 📦 Compilación para Múltiples Plataformas

### El Proyecto Soporta:

| Plataforma | Archivo | Instrucciones |
|-----------|---------|----------------|
| 🪟 Windows | .exe | Ver MULTIPLATFORM_BUILD.md |
| 🍎 macOS | Mach-O | Ver MULTIPLATFORM_BUILD.md |
| 📱 iOS | .dylib | Ver MULTIPLATFORM_BUILD.md |
| 🤖 Android | .so | Ver MULTIPLATFORM_BUILD.md |
| 🐧 Linux | ELF | Ver MULTIPLATFORM_BUILD.md |
| 🌐 ChromeOS | WASM/App | Ver MULTIPLATFORM_BUILD.md |

Para instrucciones detalladas de compilación para cada plataforma, consulta:
- [**MULTIPLATFORM_BUILD.md**](MULTIPLATFORM_BUILD.md) - Guía completa de compilación cruzada
- [**PLATFORMS.md**](PLATFORMS.md) - Matriz de compatibilidad y requisitos
- [**compile-multiplatform.ps1**](compile-multiplatform.ps1) - Script automático para Windows

## Estructura del Proyecto

### Lexer (`src/core/lexer/mod.rs`)
- Tokenización de código fuente
- Soporte para comentarios (línea y bloque)
- Manejo de strings y números

### Parser (`src/core/parser/mod.rs`)
- Análisis sintáctico recursivo descendente
- Construcción del AST
- Manejo de precedencia de operadores

### AST (`src/core/ast/mod.rs`)
- Definición universal de nodos
- Tipos de datos comunes
- Operadores y estructuras de control

### Transpiler (`src/core/transpiler/mod.rs`)
- Generación de código específica por lenguaje
- Indentación automática
- Conversión de tipos y operadores

## Próximas Características

- [ ] Soporte para clases y POO
- [ ] Mejor manejo de tipos en generación C
- [ ] Compilador integrado para validación
- [ ] GUI web
- [ ] API REST
- [ ] Optimización de código generado

## Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## Licencia

Este proyecto está bajo licencia MIT. Ver el archivo `LICENSE` para más detalles.

## Autor

University of Rovira i Virgili (URV) - Fundamentos de Programación II

---

**Nota**: Este es un proyecto educativo desarrollado para demostrar conceptos de compiladores y traductores de lenguajes.
