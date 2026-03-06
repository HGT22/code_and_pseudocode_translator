#!/usr/bin/env powershell
# Script de compilación multiplataforma para Windows
# Compila para los targets disponibles en este sistema

param(
    [string]$Profile = "release",
    [switch]$AllTargets = $false,
    [switch]$ShowHelp = $false
)

# Colores
$colors = @{
    Success = 'Green'
    Error = 'Red'
    Warning = 'Yellow'
    Info = 'Cyan'
    Header = 'Magenta'
}

function Write-Header {
    param([string]$Text)
    Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor $colors.Header
    Write-Host "║ $Text" -ForegroundColor $colors.Header
    Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor $colors.Header
}

function Write-Section {
    param([string]$Text)
    Write-Host "`n▶️  $Text" -ForegroundColor $colors.Info
    Write-Host "─────────────────────────────────────────" -ForegroundColor $colors.Info
}

function Write-Success {
    param([string]$Text)
    Write-Host "✅ $Text" -ForegroundColor $colors.Success
}

function Write-Error-msg {
    param([string]$Text)
    Write-Host "❌ $Text" -ForegroundColor $colors.Error
}

function Write-Warning-msg {
    param([string]$Text)
    Write-Host "⚠️  $Text" -ForegroundColor $colors.Warning
}

if ($ShowHelp) {
    Write-Host @"
╔════════════════════════════════════════════════════════════════════╗
║    CODE TRANSLATOR - MULTIPLATFORM COMPILATION SCRIPT             ║
╚════════════════════════════════════════════════════════════════════╝

USAGE:
  .\compile-multiplatform.ps1 [OPTIONS]

OPTIONS:
  -Profile <profile>      Profile de compilación: debug, release (default), release-mobile
  -AllTargets            Intenta compilar todos los targets posibles
  -ShowHelp              Muestra este mensaje

EXAMPLES:
  # Compilar para Windows en release
  .\compile-multiplatform.ps1 -Profile release

  # Compilar en debug
  .\compile-multiplatform.ps1 -Profile debug

  # Intenta compilar todo lo posible
  .\compile-multiplatform.ps1 -AllTargets

TARGETS SOPORTADOS EN WINDOWS:
  ✓ x86_64-pc-windows-msvc       (Windows 64-bit)
  ✓ i686-pc-windows-msvc         (Windows 32-bit)
  ◊ aarch64-pc-windows-msvc      (Windows ARM64)

TARGETS DISPONIBLES (CON CROSS-COMPILE):
  • aarch64-linux-android        (Android 64-bit)
  • armv7-linux-androideabi      (Android 32-bit)
  • x86_64-unknown-linux-gnu     (Linux)
  • x86_64-apple-darwin          (macOS Intel - requiere macOS)
  • aarch64-apple-darwin         (macOS Apple Silicon - requiere macOS)
  • aarch64-apple-ios            (iOS - requiere macOS)
  • wasm32-unknown-unknown       (WebAssembly para ChromeOS Web)

"@
    exit 0
}

# Header
Write-Header "CODE TRANSLATOR - MULTIPLATFORM BUILD"

# Verificar Rust
Write-Section "Verificando ambiente Rust"

$rustcVersion = rustc --version 2>$null
$cargoVersion = cargo --version 2>$null

if ($null -eq $rustcVersion) {
    Write-Error-msg "Rust no está instalado"
    exit 1
}

Write-Success "Rust: $rustcVersion"
Write-Success "Cargo: $cargoVersion"

# Targets disponibles
Write-Section "Listando targets instalados"

$availableTargets = @()
$output = cargo build --target-list 2>$null | Out-String

$windowsTargets = @(
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    "aarch64-pc-windows-msvc"
)

foreach ($target in $windowsTargets) {
    if ($output -match $target) {
        Write-Success "✓ $target"
        $availableTargets += $target
    } else {
        Write-Warning-msg "✗ $target (no instalado)"
    }
}

# Compilar en Windows
Write-Section "Compilando para Windows"

foreach ($target in $availableTargets) {
    $targetDisplay = $target -replace "x86_64-pc-windows-msvc", "Windows x86_64" `
                             -replace "i686-pc-windows-msvc", "Windows i686" `
                             -replace "aarch64-pc-windows-msvc", "Windows ARM64"
    
    Write-Host "📦 Compilando $targetDisplay..." -ForegroundColor $colors.Warning
    
    try {
        if ($Profile -eq "debug") {
            cargo build --target $target
        } else {
            cargo build --profile $Profile --target $target
        }
        Write-Success "Completado: $targetDisplay"
    } catch {
        Write-Error-msg "Error compilando: $targetDisplay"
    }
}

# Build Mobile optimizado (release-mobile)
if ($Profile -ne "debug") {
    Write-Section "Compilación optimizada (tamaño mínimo)"
    
    Write-Host "📦 Compilando Windows (release-mobile)..." -ForegroundColor $colors.Warning
    try {
        cargo build --profile release-mobile --target x86_64-pc-windows-msvc
        Write-Success "Completado: Windows (optimizado)"
    } catch {
        Write-Warning-msg "release-mobile no disponible, usando release"
    }
}

# Android (si está disponible cargo-ndk)
if ($AllTargets) {
    Write-Section "Intentando compilar Android"
    
    $ndkInstalled = @(cargo ndk 2>/dev/null) -ne $null
    if ($ndkInstalled) {
        Write-Host "📦 Compilando Android ARM64..." -ForegroundColor $colors.Warning
        cargo ndk -t arm64-v8a build --profile $Profile
        Write-Success "Android ARM64"
    } else {
        Write-Warning-msg "cargo-ndk no está instalado"
        Write-Host "   Para instalar: cargo install cargo-ndk" -ForegroundColor Gray
    }
}

# WebAssembly (si está disponible wasm-pack)
if ($AllTargets) {
    Write-Section "Intentando compilar WebAssembly"
    
    $wasmPackInstalled = Get-Command wasm-pack -ErrorAction SilentlyContinue
    if ($wasmPackInstalled) {
        Write-Host "📦 Compilando WebAssembly..." -ForegroundColor $colors.Warning
        wasm-pack build --target web --profile $Profile 2>$null
        Write-Success "WebAssembly compilado"
    } else {
        Write-Warning-msg "wasm-pack no está instalado"
        Write-Host "   Para instalar: cargo install wasm-pack" -ForegroundColor Gray
    }
}

# Resumen
Write-Section "RESUMEN DE COMPILACIÓN"

Write-Host @"
BINARIOS GENERADOS:

📁 Windows:
   └─ Default:  .\target\x86_64-pc-windows-msvc\$Profile\code-translator.exe
   └─ 32-bit:   .\target\i686-pc-windows-msvc\$Profile\code-translator.exe

ℹ️  Otros targets requieren compilación cruzada:
   • Android:  Usar 'cargo ndk -t arm64-v8a build --release'
   • iOS:      Disponible solo en macOS
   • Linux:    Disponible solo en Linux
   • macOS:    Disponible solo en macOS

📘 Para más información:
   • Ver MULTIPLATFORM_BUILD.md
   • Ver README.md

"@

Write-Success "¡Compilación completada!"

Write-Host @"

PRÓXIMOS PASOS:
  1. Ejecutar: cargo run
  2. Empaquetar: Ver build-guide.md
  3. Distribuir: Ver distribución.md

"@
