# 📱 Code Translator para Android

Aplicación Android nativa con interfaz gráfica que utiliza la biblioteca Rust de Code Translator.

## 🛠️ Requisitos

### Para compilar localmente:
- **Android Studio** (con Android SDK y NDK r26c)
- **Rust** (con targets de Android)
- **JDK 17** o superior

### Para usuarios finales:
- **Android 7.0 (API 24)** o superior
- Aproximadamente 15 MB de espacio

## 📦 Descarga

Descarga el APK directamente desde [GitHub Releases](https://github.com/HGT22/code_and_pseudocode_translator/releases/latest):

**[⬇️ Descargar code-translator.apk](https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator.apk)**

## 🔨 Compilar desde código fuente

### Método 1: Usando el script automático (Windows)

```powershell
.\build-android.ps1
cd android
.\gradlew.bat assembleRelease
```

El APK estará en: `android\build\outputs\apk\release\app-release.apk`

### Método 2: Usando GitHub Actions

Simplemente haz push de un tag y GitHub compilará automáticamente:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Método 3: Paso a paso manual

#### 1. Instalar Rust targets de Android

```bash
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

#### 2. Configurar Android NDK

Descarga el NDK r26c desde [Android NDK Downloads](https://developer.android.com/ndk/downloads) y configura la variable de entorno:

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk-r26c
```

#### 3. Compilar bibliotecas Rust

```bash
cargo build --release --target armv7-linux-androideabi --lib
cargo build --release --target aarch64-linux-android --lib
cargo build --release --target i686-linux-android --lib
cargo build --release --target x86_64-linux-android --lib
```

#### 4. Copiar bibliotecas a jniLibs

```bash
mkdir -p android/src/main/jniLibs/{armeabi-v7a,arm64-v8a,x86,x86_64}

cp target/armv7-linux-androideabi/release/libcode_translator.so \
   android/src/main/jniLibs/armeabi-v7a/libcode_translator_android.so

cp target/aarch64-linux-android/release/libcode_translator.so \
   android/src/main/jniLibs/arm64-v8a/libcode_translator_android.so

cp target/i686-linux-android/release/libcode_translator.so \
   android/src/main/jniLibs/x86/libcode_translator_android.so

cp target/x86_64-linux-android/release/libcode_translator.so \
   android/src/main/jniLibs/x86_64/libcode_translator_android.so
```

#### 5. Compilar APK con Gradle

```bash
cd android
./gradlew assembleRelease
```

El APK firmado estará en: `build/outputs/apk/release/app-release.apk`

## 📖 Uso

1. Abre la aplicación **Code Translator**
2. Selecciona el **lenguaje origen** del código
3. Selecciona el **lenguaje destino**
4. Introduce tu código en el área de texto
5. Pulsa **Traducir**
6. El resultado aparecerá en el área de resultado

### Lenguajes soportados

La app soporta los mismos 29 lenguajes que la versión de escritorio:

- **Lenguajes compilados**: C, C++, C#, Java, Rust, Swift, Kotlin, Go, Scala, D
- **Lenguajes interpretados**: Python, JavaScript, TypeScript, Ruby, PHP, R, Elixir
- **Lenguajes funcionales**: Haskell, F#, Elixir
- **Lenguajes de bajo nivel**: Assembly (x86/x64/ARM), WebAssembly (WAT)
- **Otros**: SQL, MATLAB, Dart
- **Pseudocódigo**: Genérico, Orientado a C (URV), Orientado a Java, Orientado a Python

### Soporte Catalan URV

La app reconoce automáticamente pseudocódigo en catalán según las directrices de la URV:
- `FUNCIÓ` → función
- `SI...ALESHORES...SINÓ...FI SI` → condicionales
- `MENTRE...FI MENTRE` → bucles while
- `PER...FI PER` → bucles for
- `RETORNAR` → return

## 🔧 Arquitectura técnica

- **Frontend**: Kotlin + Android Material Design
- **Backend**: Biblioteca Rust compilada a `libcode_translator_android.so`
- **Comunicación**: JNI (Java Native Interface)
- **Arquitecturas soportadas**: ARM 32/64-bit, x86 32/64-bit

## 📄 Licencia

Mismo que el proyecto principal. Ver [LICENSE](../LICENSE) en la raíz del repositorio.

## 🐛 Reportar problemas

Si encuentras errores o tienes sugerencias, abre un [issue en GitHub](https://github.com/HGT22/code_and_pseudocode_translator/issues).

## 👨‍💻 Autor

**Héctor García de la Torre**  
Universitat Rovira i Virgili (URV)  
Fundamentos de Programación II - 2025/26
