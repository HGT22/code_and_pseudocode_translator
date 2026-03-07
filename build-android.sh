#!/usr/bin/env bash
# Script para compilar la biblioteca Rust para Android

set -e

echo "🔨 Compilando Code Translator para Android..."

# Instalar targets de Android si no están instalados
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

# Compilar para cada arquitectura
echo "📱 Compilando para ARM 32-bit..."
cargo build --release --target armv7-linux-androideabi --lib

echo "📱 Compilando para ARM 64-bit..."
cargo build --release --target aarch64-linux-android --lib

echo "📱 Compilando para x86 32-bit..."
cargo build --release --target i686-linux-android --lib

echo "📱 Compilando para x86 64-bit..."
cargo build --release --target x86_64-linux-android --lib

# Crear directorios jniLibs
echo "📦 Copiando bibliotecas a android/src/main/jniLibs..."
mkdir -p android/src/main/jniLibs/armeabi-v7a
mkdir -p android/src/main/jniLibs/arm64-v8a
mkdir -p android/src/main/jniLibs/x86
mkdir -p android/src/main/jniLibs/x86_64

# Copiar las bibliotecas compiladas
cp target/armv7-linux-androideabi/release/libcode_translator.so android/src/main/jniLibs/armeabi-v7a/libcode_translator_android.so
cp target/aarch64-linux-android/release/libcode_translator.so android/src/main/jniLibs/arm64-v8a/libcode_translator_android.so
cp target/i686-linux-android/release/libcode_translator.so android/src/main/jniLibs/x86/libcode_translator_android.so
cp target/x86_64-linux-android/release/libcode_translator.so android/src/main/jniLibs/x86_64/libcode_translator_android.so

echo "✅ Compilación Android completada"
echo "📂 Bibliotecas en: android/src/main/jniLibs/"
echo ""
echo "Ahora puedes compilar el APK con:"
echo "  cd android && ./gradlew assembleRelease"
