# 🎯 Guía Completa: Compilar Ejecutables Multiplataforma

Esta guía te ayudará a generar ejecutables para **Windows**, **Linux**, **macOS** y **Android APK** desde tu PC con Windows.

## 📋 Estado Actual

✅ **Completado mientras dormías**:
- Estructura completa de Android creada
- App Android con interfaz gráfica (Kotlin + Material Design)
- Integración FFI Rust ↔ Java/Android
- Scripts de compilación automática (PowerShell y Bash)
- GitHub Actions actualizado para compilar APK automáticamente
- Documentación completa en `android/README.md`

⏳ **En progreso**:
- Instalación de Microsoft Visual C++ Build Tools (puede tardar ~15 minutos)
  - **Verifica**: Abre una nueva terminal y ejecuta `link.exe /?`
  - Si muestra ayuda de Microsoft LINK, está instalado ✅
  - Si dice "no se reconoce", reinicia el PC y verifica de nuevo

## 🚀 Opción 1: GitHub Actions (RECOMENDADO - Más fácil)

Esta es la forma **más sencilla** porque GitHub compila todo automáticamente en la nube.

### Paso 1: Subir todo a GitHub

```powershell
# Configura PATH de Git
$env:Path = "C:\Program Files\Git\cmd;$env:Path"

# Navega al proyecto
cd "C:\Users\Usuari\OneDrive\Escritorio\URV\2025-26\Programming\Fundamentos de Programacion\Fundamentos de  Programacion II\Code_Translator\GitHubHGT\Code Translator"

# Añade todos los archivos nuevos
git add .

# Commit
git commit -m "Add Android support and build infrastructure"

# Push
git push
```

### Paso 2: Crear y publicar tag v0.1.0

```powershell
# Crear tag
git tag -a v0.1.0 -m "v0.1.0: Windows, macOS, Linux y Android"

# Push del tag (esto activará GitHub Actions)
git push origin v0.1.0
```

### Paso 3: Esperar a que GitHub compile todo

1. Ve a: https://github.com/HGT22/code_and_pseudocode_translator/actions
2. Verás un workflow ejecutándose (tarda ~10-15 minutos)
3. Cuando termine, ve a: https://github.com/HGT22/code_and_pseudocode_translator/releases
4. Verás el release v0.1.0 con **5 archivos**:
   - ✅ `code-translator-windows-x64.exe`
   - ✅ `code-translator-macos-arm64`
   - ✅ `code-translator-macos-x64`
   - ✅ `code-translator-linux-x64`
   - ✅ `code-translator.apk` ← **ANDROID**

**¡Listo!** Todos los enlaces de descarga en el README funcionarán automáticamente.

---

## 🔧 Opción 2: Compilar Localmente (Avanzado)

Si prefieres compilar tú mismo en tu PC:

### A. Windows .exe (Nativo)

#### 1. Verifica que MSVC Build Tools está instalado

```powershell
# Abre una NUEVA terminal PowerShell
link.exe /?
```

**Si dice "no se reconoce"**:
- Opción A: Reinicia el PC (la instalación necesita reinicio)
- Opción B: Instala manualmente desde: https://visualstudio.microsoft.com/downloads/
  - Descarga "Build Tools for Visual Studio 2022"
  - Instala con workload "Desarrollo de escritorio con C++"

**Si muestra ayuda de Microsoft LINK**: ✅ Listo para compilar

#### 2. Compila el ejecutable

```powershell
# Configura PATH de Rust
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

# Navega al proyecto usando la unidad virtual (evita espacios en nombres)
cd W:\

# Compila
cargo build --release

# El ejecutable está en:
# W:\target\release\code-translator.exe
```

#### 3. Copia el ejecutable

```powershell
Copy-Item "W:\target\release\code-translator.exe" "W:\code-translator-windows-x64.exe"
```

### B. Linux (Cross-compilation)

Para compilar binarios de Linux desde Windows necesitas **cross** (usa Docker):

#### 1. Instala Docker Desktop

Descarga desde: https://www.docker.com/products/docker-desktop/

#### 2. Instala cross

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo install cross --git https://github.com/cross-rs/cross
```

#### 3. Compila para Linux

```powershell
cd W:\
cross build --release --target x86_64-unknown-linux-gnu

# El binario está en:
# W:\target\x86_64-unknown-linux-gnu\release\code-translator
```

### C. macOS (Solo en Mac o GitHub Actions)

**No es posible desde Windows** porque requiere el SDK de macOS (propietario de Apple).

**Soluciones**:
1. ✅ Usa GitHub Actions (recomendado)
2. Pide a alguien con Mac que compile
3. Usa un servicio de CI/CD (GitHub Actions ya lo hace gratis)

### D. Android APK (Requiere Android NDK)

#### 1. Instala Android Studio

Descarga desde: https://developer.android.com/studio

Durante la instalación, asegúrate de instalar:
- Android SDK
- Android NDK (r26c)

#### 2. Configura variables de entorno

```powershell
# Añade al PATH
$env:ANDROID_HOME = "C:\Users\Usuari\AppData\Local\Android\Sdk"
$env:ANDROID_NDK_HOME = "$env:ANDROID_HOME\ndk\26.3.11579264"
$env:Path = "$env:ANDROID_NDK_HOME\toolchains\llvm\prebuilt\windows-x86_64\bin;$env:Path"
```

#### 3. Ejecuta el script de compilación

```powershell
cd W:\
.\build-android.ps1
```

Este script:
- Instala targets de Rust para Android
- Compila bibliotecas `.so` para 4 arquitecturas (ARM 32/64, x86 32/64)
- Las copia a `android/src/main/jniLibs/`
- Instrucciones para compilar el APK con Gradle

#### 4. Compila el APK

```powershell
cd W:\android

# Descarga Gradle wrapper (primera vez)
# Necesitas tener JDK 17 instalado
.\gradlew.bat assembleRelease

# El APK estará en:
# build\outputs\apk\release\app-release.apk
```

---

## 🎯 Comparación: GitHub Actions vs Local

| Criterio | GitHub Actions | Compilación Local |
|----------|---------------|-------------------|
| **Facilidad** | ⭐⭐⭐⭐⭐ Solo push tag | ⭐⭐ Requiere setup |
| **Tiempo** | 10-15 min (automático) | 30-60 min (manual) |
| **Plataformas** | Todos (Win/Mac/Linux/Android) | Solo Win + Linux (con cross) |
| **Requisitos** | Solo Git | MSVC, NDK, Docker, etc. |
| **Costo** | Gratis | Gratis |
| **Recomendado para** | **Primera release, distribución** | Testing local |

## ✅ Recomendación Final

Para tu **primer release público**:

1. **Usa GitHub Actions** (Opción 1)
   - Menos problemas
   - Menos tiempo
   - Compila todo automáticamente (incluso macOS)

2. **Compilación local** es útil para:
   - Testing rápido en Windows
   - Desarrollo iterativo
   - Cuando no quieres hacer push todavía

## 🐛 Solución de Problemas Comunes

### "cargo: comando no encontrado"

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
rustup --version
```

### "linking with link.exe failed"

- MSVC Build Tools no instalado correctamente
- **Solución**: Reinicia el PC o instala desde Visual Studio Installer

### "NDK not found" (Android)

```powershell
# Verifica instalación NDK
ls "$env:ANDROID_HOME\ndk"

# Si está vacío, instala desde Android Studio:
# Tools → SDK Manager → SDK Tools → NDK (Side by side)
```

### "Gradle build failed"

```powershell
# Verifica JDK 17
java -version

# Si no lo tienes, descarga:
# https://adoptium.net/temurin/releases/?version=17
```

## 📞 Siguiente Paso

**Cuando despiertes**, decide:

- **Quiero distribución rápida**: Ejecuta Opción 1 (GitHub Actions) - 5 minutos
- **Quiero aprender el proceso**: Sigue Opción 2 (Local) - 1-2 horas

**Mi recomendación**: Empieza con Opción 1 para publicar rápido, luego experimenta con Opción 2.

---

**Estado de la instalación MSVC**: 
- Proceso iniciado a las ~23:30
- Terminal ID: `248896a0-644b-46da-85a4-0948262c9d59`
- Comando: `winget install Microsoft.VisualStudio.2022.BuildTools`
- **Al despertar**: Abre PowerShell y ejecuta `link.exe /?` para verificar

¡Buenas noches! 🌙
