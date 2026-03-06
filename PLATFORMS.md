# 🌍 Sistemas Operativos Soportados

Guía rápida sobre compatibilidad y compilación para todos los SOs soportados.

---

## 📊 Matriz de Compatibilidad

```
┌─────────────┬──────────┬──────────┬──────────┬──────────┐
│ Sistema     │ Versión  │ State    │ Binario  │ Nivel    │
├─────────────┼──────────┼──────────┼──────────┼──────────┤
│ Windows     │ 10/11    │ ✅ Full  │ .exe     │ Tier 1   │
│ macOS       │ 10.12+   │ ✅ Full  │ Mach-O   │ Tier 1   │
│ iOS         │ 12.0+    │ ✅ Full  │ .dylib   │ Tier 1   │
│ Android     │ 5.0+     │ ✅ Full  │ .so      │ Tier 1   │
│ Linux       │ 2.6.32+  │ ✅ Full  │ ELF      │ Tier 1   │
│ ChromeOS    │ Any      │ ✅ Full  │ WASM/App │ Tier 2   │
│ iPadOS      │ 12.0+    │ ✅ Full  │ .dylib   │ Tier 1   │
│ tvOS        │ 12.0+    │ ⚠️ Beta  │ .dylib   │ Tier 2   │
│ watchOS     │ 5.0+     │ ⚠️ Beta  │ .dylib   │ Tier 3   │
└─────────────┴──────────┴──────────┴──────────┴──────────┘
```

---

## 🪟 Windows

**Estado**: ✅ Completamente Soportado (Tier 1)

### Versiones Soportadas
- Windows 10 (1909+)
- Windows 11 (todas las ediciones)
- Windows Server 2019+

### Arquitecturas
- ✅ x86_64 (64-bit)
- ✅ i686 (32-bit)
- ⚠️ aarch64 (ARM64 - experimental)

### Requisitos
- Visual Studio Build Tools 2019+ o Visual Studio Community
- Rust Toolchain
- 2GB RAM libre, 1GB disk space

### Instalación Rápida
```powershell
# Descargar e instalar Rust
Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe
.\rustup-init.exe

# Compilar
cargo build --release

# Ejecutar
.\target\release\code-translator.exe
```

### Distribución
- Ejecutable standalone: code-translator.exe
- Instalador MSI (próximamente)
- Microsoft Store (próximamente)

---

## 🍎 macOS

**Estado**: ✅ Completamente Soportado (Tier 1)

### Versiones Soportadas
- macOS 10.12+ (Sierra)
- macOS 12+ (Monterey y posterior recomendado)
- macOS 13 (Ventura)
- macOS 14 (Sonoma)

### Arquitecturas
- ✅ x86_64 (Intel)
- ✅ aarch64 (Apple Silicon M1/M2/M3+)
- ✅ Universal Binary (ambas arquitecturas)

### Requisitos
- Xcode Command Line Tools
- Rust Toolchain
- 2GB RAM, 1GB disk space

### Instalación Rápida
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compilar
cargo build --release

# Ejecutar
./target/release/code-translator
```

### Distribución
- .app bundle para Mac App Store
- .dmg para distribución directa
- Universal binary (Intel + Silicon)

### Notarización
```bash
# Firmar y notarizar para macOS Gatekeeper
codesign -s - target/release/code-translator
xcrun altool --notarize-app -f target/release/code-translator \
    -t osx --file-type application \
    -u my-apple-id@example.com
```

---

## 🍏 iOS

**Estado**: ✅ Completamente Soportado (Tier 1)

### Versiones Soportadas
- iOS 12.0+
- Todo iPhone y iPad moderno

### Arquitecturas
- ✅ aarch64 (ARM64 - dispositivos)
- ✅ aarch64 (Simulador)
- ✅ x86_64 (Simulador antiguo)

### Requisitos
- macOS con Xcode
- Rust + iOS targets
- iPhone/iPad o simulador

### Compilación
```bash
# Agregar targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# Compilar para dispositivo
cargo build --release --target aarch64-apple-ios

# Compilar para simulador
cargo build --release --target aarch64-apple-ios-sim
```

### Distribución
- App Store (requiere Apple Developer Program)
- TestFlight (distribución beta)
- Ad-hoc (pruebas internas)

### Crear App
```bash
# Crear framework
mkdir -p CodeTranslator.framework/Headers

lipo -create \
    target/aarch64-apple-ios/release/libcode_translator.a \
    target/aarch64-apple-ios-sim/release/libcode_translator.a \
    -output CodeTranslator.framework/CodeTranslator
```

---

## 🤖 Android

**Estado**: ✅ Completamente Soportado (Tier 1)

### Versiones Soportadas
- Android 5.0+ (API level 21+)
- >95% de dispositivos Android actuales

### Arquitecturas
- ✅ arm64-v8a (ARMv8 64-bit)
- ✅ armeabi-v7a (ARMv7 32-bit)
- ✅ x86 (Emulador)
- ✅ x86_64 (Emulador)

### Requisitos
- Android NDK (descargar)
- Rust + Android targets
- Android Studio (recomendado)

### Compilación
```bash
# Instalar targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Instalar cargo-ndk
cargo install cargo-ndk

# Compilar
cargo ndk -t arm64-v8a -t armeabi-v7a build --release
```

### Distribución
- Google Play Store
- Samsung Galaxy Store
- F-Droid
- APK directo

### Crear APK
- Usar Android Studio con Gradle
- O: `bundletool` para crear AAB

---

## 🐧 Linux

**Estado**: ✅ Completamente Soportado (Tier 1)

### Distribuciones Soportadas
- Ubuntu 18.04+ (LTS)
- Debian 10+
- Fedora 32+
- Arch Linux
- Alpine Linux
- Red Hat/CentOS 7+
- SUSE Linux Enterprise

### Arquitecturas
- ✅ x86_64 (Intel/AMD 64-bit)
- ✅ i686 (Intel/AMD 32-bit)
- ✅ aarch64 (ARM 64-bit)
- ✅ armv7 (ARM 32-bit)
- ✅ armv6 (Raspberry Pi 1/Zero)

### Requisitos
- GCC/Clang + build-essential
- 1GB RAM, 500MB disk space

### Compilación
```bash
# Ubuntu/Debian
sudo apt-get install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compilar
cargo build --release

# Ejecutar
./target/release/code-translator
```

### Distribución
- .deb (Debian/Ubuntu)
- .rpm (Fedora/Red Hat)
- Snap
- AppImage
- Arch User Repository (AUR)

### Crear .deb
```bash
mkdir -p deb/usr/local/bin
cp target/release/code-translator deb/usr/local/bin/
dpkg-deb --build deb code-translator.deb
```

---

## 🌐 ChromeOS

**Estado**: ✅ Completamente Soportado (Tier 1)

### Métodos Disponibles

### Opción 1: Crostini (Linux Containers)
- Ejecutar versión Linux directamente
- Acceso completo a terminal
- Mejor rendimiento

```bash
# Dentro de contenedor Linux en ChromeOS
sudo apt-get install cargo
cargo build --release
./target/release/code-translator
```

### Opción 2: Web (WebAssembly)
- PWA (Progressive Web App)
- Funciona offline
- Sincroniza en la nube

```bash
# Compilar a WASM
cargo install wasm-pack
wasm-pack build --target web --release
```

### Opción 3: Android (Play Store)
- Instalar desde Google Play Store
- Funciona como app nativa

### Requisitos (por método)
- **Crostini**: ChromeOS 69+, 4GB RAM
- **Web**: Cualquier ChromeOS, navegador moderno
- **Android**: ChromeOS 80+ con soporte Android

### Estado Crostini
```bash
chrome://crostini
```

---

## 📱 iPadOS

**Estado**: ✅ Completamente Soportado (Tier 1)

### Versiones Soportadas
- iPadOS 12.0+
- Todos los iPad (Air 2+, Pro, Mini 4+)

### Notas
- Usa el mismo código que iOS
- Interfaz optimizada para tablet
- Split-screen compatible

### Compilación
- Igual a iOS (mismo target: aarch64-apple-ios)
- Detecta automáticamente en runtime

---

## 📺 tvOS

**Estado**: ⚠️ En Beta (Tier 2)

### Versiones Soportadas
- tvOS 12.0+

### Estado
- ⚠️ Compilación posible
- ⚠️ No completamente testeado
- ⚠️ Interfaz límitada (sin pantalla táctil)

### Compilar
```bash
rustup target add aarch64-apple-tvos
cargo build --release --target aarch64-apple-tvos
```

---

## ⌚ watchOS

**Estado**: ⚠️ En Beta (Tier 3)

### Versiones Soportadas
- watchOS 5.0+

### Limitaciones
- ⚠️ Pantalla muy pequeña
- ⚠️ Limitaciones de memoria
- ⚠️ No recomendado para código complejo

---

## 🔄 Tabla de Compilación Cruzada

| Compilar desde | Windows | macOS | Linux |
|---|---|---|---|
| **Windows** | ✅ Nativa | ❌ No | ❌ No |
| **macOS** | ❌ Limitado | ✅ Nativa | ❌ No |
| **Linux** | ✅ Con mingw | ✅ Con osxcross | ✅ Nativa |

---

## 📥 Descargas Recomendadas

| SO | Recomendación | Alternativa |
|---|---|---|
| Windows | Store/Web | GitHub Releases |
| macOS | App Store | DMG directo |
| iOS | App Store | TestFlight |
| Android | Google Play | APK directo |
| Linux | Distro package | GitHub Releases |
| ChromeOS | Play Store | Web app |

---

## 🆘 Solución de Problemas

### Windows
- ❌ "cargo: command not found" → Reiniciar PowerShell
- ❌ Error MSVC → Instalar Visual Studio Build Tools

### macOS
- ❌ "xcode-select: error" → `xcode-select --install`
- ❌ Permisos → `sudo chown -R $(whoami) ~/.cargo`

### Linux
- ❌ "command not found: cargo" → `source ~/.cargo/env`
- ❌ Linker error → `sudo apt-get install build-essential`

### iOS/Android
- ❌ Permisos → Ver documentación de App Store/Play Store
- ❌ Certificados → Configurar en Xcode/Android Studio

---

## 📈 Roadmap de Plataformas

### Tier 1 (Producción)
✅ Windows | ✅ macOS | ✅ Linux | ✅ iOS | ✅ Android | ✅ ChromeOS

### Tier 2 (Experimental)
⚠️ tvOS | 🔜 FreeBSD | 🔜 NetBSD

### Tier 3 (Futuro)
🔜 watchOS | 🔜 VisionOS | 🔜 Haiku OS

---

**Última actualización**: Marzo 2026
**Plataformas soportadas**: 6 (8 con beta)
**Total de arquitecturas**: 9+
