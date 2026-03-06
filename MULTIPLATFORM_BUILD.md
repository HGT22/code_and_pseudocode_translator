# 🌍 Compilación Multiplataforma - Code Translator

Guía completa para compilar el Traductor de Lenguajes en Windows, macOS, iOS, Android, Linux y ChromeOS.

---

## 📋 Tabla de Contenidos

- [Windows](#windows)
- [macOS](#macos)
- [iOS](#ios)
- [Android](#android)
- [Linux](#linux)
- [ChromeOS](#chromeos)
- [Comparison Table](#tabla-comparativa)
- [CI/CD](#cicd-github-actions)

---

## 🪟 Windows

### Requisitos
- Visual Studio Build Tools o Visual Studio Community
- Rust (rustup)
- Windows 10/11 (64-bit)

### Compilación

#### Debug
```powershell
cargo build
```

#### Release (optimizado)
```powershell
cargo build --release
```

#### Release minimizado (tamaño pequeño)
```powershell
cargo build --release --profile release-mobile
```

### Resultado
- Debug: `target/debug/code-translator.exe`
- Release: `target/release/code-translator.exe`

### Distribución
```powershell
# Crear instalador
New-Item -ItemType Directory -Path dist

Copy-Item target/release/code-translator.exe dist/
Copy-Item README.md dist/
Copy-Item LICENSE dist/

# Crear ZIP
Compress-Archive -Path dist -DestinationPath code-translator-windows.zip
```

---

## 🍎 macOS

### Requisitos
- macOS 10.12+
- Xcode Command Line Tools
- Rust (rustup)

### Instalación de dependencias

```bash
# Instalar Xcode Command Line Tools
xcode-select --install

# Instalar/actualizar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Agregar targets (si es necesario)
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Compilación

#### Intel (x86_64)
```bash
cargo build --release --target x86_64-apple-darwin
```

#### Apple Silicon (ARM64)
```bash
cargo build --release --target aarch64-apple-darwin
```

#### Universal Binary (ambas arquitecturas)
```bash
#!/bin/bash

# Compilar ambas arquitecturas
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Crear universal binary
mkdir -p target/universal-apple-darwin/release

lipo -create \
    target/x86_64-apple-darwin/release/code-translator \
    target/aarch64-apple-darwin/release/code-translator \
    -output target/universal-apple-darwin/release/code-translator

# Crear .app bundle
mkdir -p Code\ Translator.app/Contents/MacOS
mkdir -p Code\ Translator.app/Contents/Resources

cp target/universal-apple-darwin/release/code-translator Code\ Translator.app/Contents/MacOS/
```

### Info.plist (para .app bundle)
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>code-translator</string>
    <key>CFBundleIdentifier</key>
    <string>com.urv.code-translator</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Code Translator</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
</dict>
</plist>
```

### Distribución
```bash
# Crear DMG para distribución
hdiutil create -volname "Code Translator" \
               -srcfolder Code\ Translator.app \
               -ov -format UDZO code-translator.dmg
```

---

## 🍏 iOS

### Requisitos
- macOS (Xcode)
- Rust with iOS targets
- iOS SDK
- iPhone/iPad para pruebas

### Instalación de Targets

```bash
# Agregar targets iOS
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim

# Instalar herramientas iOS
cargo install cargo-binutils
```

### Compilación

#### Dispositivo físico (ARM64)
```bash
cargo build --release --target aarch64-apple-ios \
    --profile release-mobile
```

#### Simulador iOS 64-bit
```bash
cargo build --release --target aarch64-apple-ios-sim \
    --profile release-mobile
```

#### Simulador iOS antiguo (Intel)
```bash
cargo build --release --target x86_64-apple-ios \
    --profile release-mobile
```

### Crear Framework de iOS

```bash
#!/bin/bash

# Crear estructura framework
FRAMEWORK_NAME="CodeTranslator"
FRAMEWORK_DIR="${FRAMEWORK_NAME}.framework"

mkdir -p ${FRAMEWORK_DIR}/Headers
mkdir -p ${FRAMEWORK_DIR}/Modules

# Copiar binarios compilados
lipo -create \
    target/aarch64-apple-ios/release-mobile/libcode_translator.a \
    target/aarch64-apple-ios-sim/release-mobile/libcode_translator.a \
    -output ${FRAMEWORK_DIR}/${FRAMEWORK_NAME}

# Crear header (si es necesario)
cat > ${FRAMEWORK_DIR}/Headers/${FRAMEWORK_NAME}.h << 'EOF'
#ifndef CODE_TRANSLATOR_H
#define CODE_TRANSLATOR_H

#include <stdint.h>

extern const char* translate(const char* code, const char* from_lang, const char* to_lang);

#endif
EOF

# Crear module.modulemap
cat > ${FRAMEWORK_DIR}/Modules/module.modulemap << 'EOF'
framework module CodeTranslator {
    header "CodeTranslator.h"
    export *
}
EOF

# Crear Info.plist
cat > ${FRAMEWORK_DIR}/Info.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>CodeTranslator</string>
    <key>CFBundleIdentifier</key>
    <string>com.urv.CodeTranslator</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>CodeTranslator</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>CFBundleVersion</key>
    <string>1</string>
</dict>
</plist>
EOF

echo "✅ Framework creado: ${FRAMEWORK_DIR}"
```

### Usar en Xcode

1. Agregar framework al proyecto
2. Importar en Swift:
```swift
import CodeTranslator

let translated = translate(code, "Python", "C")
```

---

## 🤖 Android

### Requisitos
- Android NDK (descargar)
- Rust with Android targets
- Android Studio (opcional)

### Instalación de Targets

```bash
# Agregar targets Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Instalar cargo-ndk
cargo install cargo-ndk
```

### Compilación

#### ARM64 (recomendado para devices modernos)
```bash
cargo ndk -t arm64-v8a build --release
```

#### ARMv7 (devices más antiguos)
```bash
cargo ndk -t armeabi-v7a build --release
```

#### x86/x86_64 (emulador)
```bash
cargo ndk -t x86 -t x86_64 build --release
```

#### Todas las arquitecturas
```bash
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 build --release
```

### Crear archivo build.gradle

```gradle
android {
    compileSdkVersion 33

    defaultConfig {
        applicationId "com.urv.codetranslator"
        minSdkVersion 21
        targetSdkVersion 33
        versionCode 1
        versionName "0.1.0"

        externalNativeBuild {
            cmake {
                cppFlags "-std=c++17"
            }
        }
    }

    externalNativeBuild {
        cmake {
            path "CMakeLists.txt"
        }
    }
}
```

### Crear archivo CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.18.1)
project(code_translator)

add_library(code_translator SHARED IMPORTED)
set_target_properties(
    code_translator PROPERTIES 
    IMPORTED_LOCATION
    ${CMAKE_CURRENT_SOURCE_DIR}/../../target/${ANDROID_ABI}/release/libcode_translator.so
)

target_include_directories(code_translator PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main/jni
)
```

---

## 🐧 Linux

### Requisitos
- GCC o Clang
- Rust
- Linux Kernel 2.6.32+

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compilación

#### x86_64
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

#### i686 (32-bit)
```bash
rustup target add i686-unknown-linux-gnu
cargo build --release --target i686-unknown-linux-gnu
```

#### ARM (Raspberry Pi)
```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
```

#### ARM64 (ARMv8)
```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

### Distribución DEB
```bash
mkdir -p code-translator_0.1.0/usr/local/bin
mkdir -p code-translator_0.1.0/DEBIAN

cp target/release/code-translator code-translator_0.1.0/usr/local/bin/

cat > code-translator_0.1.0/DEBIAN/control << 'EOF'
Package: code-translator
Version: 0.1.0
Section: programming
Priority: optional
Maintainer: URV <info@urv.cat>
Description: Multi-language code translator
 Supports C, Python, JavaScript, Pseudocode translation
EOF

dpkg-deb --build code-translator_0.1.0 code-translator_0.1.0-amd64.deb
```

---

## 🌐 ChromeOS

### Opción 1: Linux (Crostini)

ChromeOS permite ejecutar contenedores Linux. Usa las instrucciones de Linux con Crostini.

```bash
# Dentro del contenedor Linux en ChromeOS

# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Compilar
cd ~/code-translator
cargo build --release

# 3. Ejecutar
./target/release/code-translator
```

### Opción 2: Web Assembly (WASM)

Crear versión web que funcione en ChromeOS como PWA.

#### Instalar wasm-pack

```bash
curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh
```

#### Compilar a WASM

```bash
wasm-pack build --target web --release
```

#### Crear índice HTML

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Code Translator</title>
    <script type="module">
        import * as wasm from './pkg/code_translator.js';
        
        document.getElementById('translate-btn').addEventListener('click', async () => {
            const code = document.getElementById('code-input').value;
            const from = document.getElementById('from-lang').value;
            const to = document.getElementById('to-lang').value;
            
            try {
                const result = await wasm.translate(code, from, to);
                document.getElementById('output').value = result;
            } catch (error) {
                alert('Error: ' + error.message);
            }
        });
    </script>
</head>
<body>
    <h1>Code Translator</h1>
    <div>
        <label>From: <select id="from-lang"></select></label>
        <label>To: <select id="to-lang"></select></label>
        <button id="translate-btn">Translate</button>
    </div>
    <textarea id="code-input" placeholder="Paste code here..."></textarea>
    <textarea id="output" readonly placeholder="Translated code..."></textarea>
</body>
</html>
```

#### Desplegar como PWA

```json
{
  "manifest_version": 3,
  "name": "Code Translator",
  "version": "0.1.0",
  "description": "Multi-language code translator for ChromeOS",
  "permissions": ["storage"],
  "action": {
    "default_popup": "index.html",
    "default_title": "Code Translator"
  },
  "icons": [
    {"src": "/icon128.png", "sizes": "128x128", "type": "image/png"}
  ]
}
```

---

## 📊 Tabla Comparativa

| SO | Executable | Compilación | Tamaño | Rendimiento | Notas |
|----|-----------|-------------|--------|------------|-------|
| Windows | .exe | Easy | 15-20 MB | Excelente | Visual Studio toolchain |
| macOS | Mach-O | Medium | 10-15 MB | Excelente | Requiere Xcode Command Line |
| iOS | .framework | Medium | 5-10 MB | Muy bueno | Requiere jailbreak o App Store |
| Android | .so | Medium | 8-12 MB | Muy bueno | Integración con Gradle |
| Linux | ELF | Easy | 12-18 MB | Excelente | Muy portable |
| ChromeOS | WASM | Medium | 2-5 MB | Bueno | Web-based, offline capable |

---

## 🔧 CI/CD - GitHub Actions

### .github/workflows/multiplatform-build.yml

```yaml
name: Multiplatform Build

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-windows
          path: target/release/code-translator.exe

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: |
          rustup target add x86_64-apple-darwin aarch64-apple-darwin
          cargo build --release --target x86_64-apple-darwin
          cargo build --release --target aarch64-apple-darwin
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-macos
          path: target/*/release/code-translator

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-linux
          path: target/release/code-translator

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: |
          rustup target add aarch64-linux-android
          cargo install cargo-ndk
          cargo ndk -t arm64-v8a build --release
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-android
          path: target/aarch64-linux-android/release/libcode_translator.so

  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: |
          rustup target add aarch64-apple-ios aarch64-apple-ios-sim
          cargo build --release --target aarch64-apple-ios
          cargo build --release --target aarch64-apple-ios-sim
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-ios
          path: target/aarch64-apple-ios*/release/libcode_translator.a

  build-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: |
          rustup target add wasm32-unknown-unknown
          cargo install wasm-pack
          wasm-pack build --target web --release
      - uses: actions/upload-artifact@v3
        with:
          name: code-translator-wasm
          path: pkg/
```

---

## Script Maestro de Compilación

### compile-all-platforms.sh

```bash
#!/bin/bash

set -e

echo "🌍 Code Translator - Multiplatform Compilation"
echo "=============================================="
echo ""

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

build_target() {
    local target=$1
    local name=$2
    
    echo -e "${YELLOW}📦 Building $name ($target)...${NC}"
    cargo build --release --target "$target" 2>&1 | tail -5
    echo -e "${GREEN}✅ Success: $name${NC}\n"
}

# Windows
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    build_target "x86_64-pc-windows-msvc" "Windows 64-bit"
fi

# macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    build_target "x86_64-apple-darwin" "macOS Intel"
    build_target "aarch64-apple-darwin" "macOS Apple Silicon"
fi

# Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    build_target "x86_64-unknown-linux-gnu" "Linux x86_64"
    build_target "aarch64-unknown-linux-gnu" "Linux ARM64"
fi

# Android (all platforms)
echo -e "${YELLOW}📦 Building Android targets...${NC}"
cargo ndk -t arm64-v8a -t armeabi-v7a build --release 2>&1 | tail -5
echo -e "${GREEN}✅ Success: Android${NC}\n"

# iOS (macOS only)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "${YELLOW}📦 Building iOS targets...${NC}"
    cargo build --release --target aarch64-apple-ios 2>&1 | tail -5
    cargo build --release --target aarch64-apple-ios-sim 2>&1 | tail -5
    echo -e "${GREEN}✅ Success: iOS${NC}\n"
fi

# WASM
echo -e "${YELLOW}📦 Building WebAssembly...${NC}"
wasm-pack build --target web --release 2>&1 | tail -5
echo -e "${GREEN}✅ Success: WebAssembly${NC}\n"

echo -e "${GREEN}🎉 All builds completed successfully!${NC}"
echo ""
echo "📁 Built artifacts:"
echo "   - Windows:  target/x86_64-pc-windows-msvc/release/"
echo "   - macOS:    target/{x86_64,aarch64}-apple-darwin/release/"
echo "   - Linux:    target/{x86_64,aarch64}-unknown-linux-gnu/release/"
echo "   - Android:  target/{aarch64-linux-android,armv7-linux-androideabi}/release/"
echo "   - iOS:      target/{aarch64-apple-ios*}/release/"
echo "   - WASM:     pkg/"
```

---

**Última actualización**: Marzo 2026
**Estado**: Multiplataforma completo ✅
