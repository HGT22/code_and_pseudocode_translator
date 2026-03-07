# Script PowerShell para compilar Code Translator para Android

Write-Host "🔨 Compilando Code Translator para Android..." -ForegroundColor Cyan

# Configurar PATH de Rust
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

# Instalar targets de Android
Write-Host "`n📥 Instalando targets de Android..." -ForegroundColor Yellow
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Compilar para cada arquitectura
Write-Host "`n📱 Compilando para ARM 32-bit (armv7)..." -ForegroundColor Green
cargo build --release --target armv7-linux-androideabi --lib

Write-Host "`n📱 Compilando para ARM 64-bit (aarch64)..." -ForegroundColor Green
cargo build --release --target aarch64-linux-android --lib

Write-Host "`n📱 Compilando para x86 32-bit..." -ForegroundColor Green
cargo build --release --target i686-linux-android --lib

Write-Host "`n📱 Compilando para x86 64-bit..." -ForegroundColor Green
cargo build --release --target x86_64-linux-android --lib

# Crear directorios jniLibs
Write-Host "`n📦 Creando estructura jniLibs..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path "android\src\main\jniLibs\armeabi-v7a" | Out-Null
New-Item -ItemType Directory -Force -Path "android\src\main\jniLibs\arm64-v8a" | Out-Null
New-Item -ItemType Directory -Force -Path "android\src\main\jniLibs\x86" | Out-Null
New-Item -ItemType Directory -Force -Path "android\src\main\jniLibs\x86_64" | Out-Null

# Copiar bibliotecas
Write-Host "📋 Copiando bibliotecas..." -ForegroundColor Cyan
Copy-Item "target\armv7-linux-androideabi\release\libcode_translator.so" "android\src\main\jniLibs\armeabi-v7a\libcode_translator_android.so" -Force
Copy-Item "target\aarch64-linux-android\release\libcode_translator.so" "android\src\main\jniLibs\arm64-v8a\libcode_translator_android.so" -Force
Copy-Item "target\i686-linux-android\release\libcode_translator.so" "android\src\main\jniLibs\x86\libcode_translator_android.so" -Force
Copy-Item "target\x86_64-linux-android\release\libcode_translator.so" "android\src\main\jniLibs\x86_64\libcode_translator_android.so" -Force

Write-Host "`n✅ Compilación Android completada" -ForegroundColor Green
Write-Host "📂 Bibliotecas en: android\src\main\jniLibs\" -ForegroundColor Yellow
Write-Host ""
Write-Host "Ahora compila el APK con:" -ForegroundColor Cyan
Write-Host "  cd android" -ForegroundColor White
Write-Host "  .\gradlew.bat assembleRelease" -ForegroundColor White
Write-Host ""
Write-Host "O usa GitHub Actions para compilarlo automáticamente" -ForegroundColor Magenta
