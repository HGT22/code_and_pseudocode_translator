# Compilación Multiplataforma

## Compilación para Windows

### Requisitos
- Visual Studio Build Tools o Visual Studio Community
- Rust toolchain instalado (con `rustup`)

### Comandos
```powershell
# Compilación de debug
cargo build

# Compilación de release (optimizada)
cargo build --release

# Ejecutable generado
.\target\release\code-translator.exe
```

---

## Compilación para Android

### Requisitos
1. **Rust con soporte Android**
```powershell
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
```

2. **Android NDK**
   - Descargar desde: https://developer.android.com/ndk
   - Mínimo: NDK r21

3. **Cargo-ndk**
```powershell
cargo install cargo-ndk
```

### Compilación
```powershell
# Configurar variables de entorno para Android NDK
$env:NDK_HOME = "C:\path\to\android-ndk"

# Compilar para ARM64 (recomendado para dispositivos modernos)
cargo ndk -t arm64-v8a build --release

# Compilar para ARMv7 (dispositivos más antiguos)
cargo ndk -t armeabi-v7a build --release

# Compilar para ambas arquitecturas
cargo ndk -t arm64-v8a -t armeabi-v7a build --release
```

### Salida
```
target/aarch64-linux-android/release/libcode_translator.so
target/armv7-linux-androideabi/release/libcode_translator.so
```

### Integración con Android Studio

#### 1. Crear proyecto Android
```bash
android create project --target android-30 --name CodeTranslator --path ./android
```

#### 2. Configurar CMakeLists.txt
```cmake
cmake_minimum_required(VERSION 3.4.1)

add_library(code_translator SHARED IMPORTED)
set_target_properties(
    code_translator PROPERTIES IMPORTED_LOCATION
    ${CMAKE_CURRENT_SOURCE_DIR}/../../target/${ANDROID_ABI}/release/libcode_translator.so)
```

#### 3. Llamar desde Java
```java
public class CodeTranslator {
    static {
        System.loadLibrary("code_translator");
    }
    
    public native String translate(String code, String sourceLanguage, String targetLanguage);
}
```

---

## Compilación para Linux

### Requisitos
```bash
sudo apt-get install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Comandos
```bash
# Compilación de release
cargo build --release

# Ejecutable
./target/release/code-translator
```

---

## Compilación con características personalizadas

### Build scripts personalizados

#### create_build_script.ps1
```powershell
# Script para compilar todas las plataformas

param(
    [string]$Configuration = "release"
)

Write-Host "🔨 Compilando Code Translator..."

# Windows
Write-Host "📦 Compilando para Windows..."
cargo build --$Configuration --target x86_64-pc-windows-msvc
if ($? -eq $false) { exit 1 }

# Android ARM64
Write-Host "📱 Compilando para Android ARM64..."
cargo ndk -t arm64-v8a build --$Configuration
if ($? -eq $false) { exit 1 }

Write-Host "✅ Compilación completada"
Write-Host "📁 Binarios disponibles en:"
Write-Host "  - Windows: target/x86_64-pc-windows-msvc/$Configuration/code-translator.exe"
Write-Host "  - Android: target/aarch64-linux-android/$Configuration/libcode_translator.so"
```

---

## Testing en múltiples plataformas

### Ejecutar tests
```bash
# Todos los tests
cargo test --release

# Tests específicos de un módulo
cargo test lexer --release
cargo test parser --release
cargo test transpiler --release
```

### Cobertura de código
```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Generar reporte de cobertura
cargo tarpaulin --out Html --output-dir coverage
```

---

## Distribución

### Empaquetamiento para Windows
```powershell
# Crear carpeta de distribución
New-Item -ItemType Directory -Path dist

# Copiar binario y recursos
Copy-Item target/release/code-translator.exe dist/
Copy-Item README.md dist/
Copy-Item EXAMPLES.md dist/
Copy-Item LICENSE dist/

# Crear ZIP
Compress-Archive -Path dist -DestinationPath code-translator-windows.zip
```

### Empaquetamiento para Android
```bash
# El archivo .so se integra en el APK mediante Android Studio
# El proceso se realiza automáticamente con Gradle
```

---

## Optimizaciones de compilación

### Perfil de release optimizado
```toml
[profile.release]
opt-level = 3          # Máxima optimización
lto = true            # Link-time optimization
codegen-units = 1     # Compilación más lenta pero optimizada
strip = true          # Eliminar símbolos de debug
```

### Size-optimized build
```toml
[profile.release-minimal]
inherits = "release"
opt-level = "z"       # Optimizar tamaño
lto = true
strip = true
```

### Construir versión pequeña
```bash
cargo build --release --config profile.release-minimal
```

---

## Solución de problemas

### Error: NDK no encontrado
```powershell
$env:NDK_HOME = "C:\path\to\ndk"
$env:Path += ";$env:NDK_HOME\bin"
```

### Error: Target no disponible
```bash
rustup target list    # Ver targets disponibles
rustup target add <target>  # Instalar target
```

### Error de permisos en Linux
```bash
chmod +x target/release/code-translator
```

---

## CI/CD con GitHub Actions

### .github/workflows/build.yml
```yaml
name: Build Multi-Platform

on: [push, pull_request]

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v2
        with:
          name: code-translator-windows
          path: target/release/code-translator.exe

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install cargo-ndk
      - run: cargo ndk -t arm64-v8a build --release
```

---

**Última actualización**: Marzo 2026
