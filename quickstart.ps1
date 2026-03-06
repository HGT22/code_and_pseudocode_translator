#!/usr/bin/env powershell
# Script de inicio rápido para Code Translator

Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   CODE TRANSLATOR - QUICK START          ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Verificar si Rust está instalado
Write-Host "🔍 Verificando instalación de Rust..." -ForegroundColor Yellow
$rustcVersion = rustc --version 2>$null
$cargoVersion = cargo --version 2>$null

if ($null -eq $rustcVersion) {
    Write-Host "❌ Rust no está instalado" -ForegroundColor Red
    Write-Host ""
    Write-Host "📖 Para instalar Rust, ve a: https://rustup.rs/" -ForegroundColor White
    Write-Host "   o ejecuta: Invoke-WebRequest -useb https://sh.rustup.rs/ | sh" -ForegroundColor Gray
    Write-Host ""
    exit 1
}

Write-Host "✅ Rust instalado: $rustcVersion" -ForegroundColor Green
Write-Host "✅ Cargo disponible: $cargoVersion" -ForegroundColor Green
Write-Host ""

# Mostrar opciones
Write-Host "¿Qué deseas hacer?" -ForegroundColor Cyan
Write-Host ""
Write-Host "  1) 🏗️  Compilar el proyecto (debug)"
Write-Host "  2) ⚡ Compilar optimizado (release)"
Write-Host "  3) ▶️  Ejecutar la aplicación"
Write-Host "  4) 🧪 Ejecutar tests"
Write-Host "  5) 📖 Ver documentación"
Write-Host "  6) 🧹 Limpiar compilaciones"
Write-Host ""

$choice = Read-Host "Selecciona una opción (1-6)"

Write-Host ""

switch ($choice) {
    "1" {
        Write-Host "🏗️  Compilando en modo debug..." -ForegroundColor Yellow
        cargo build
        Write-Host "✅ Compilación completada" -ForegroundColor Green
    }
    
    "2" {
        Write-Host "⚡ Compilando en modo release..." -ForegroundColor Yellow
        cargo build --release
        Write-Host "✅ Compilación completada" -ForegroundColor Green
        Write-Host "📁 Ejecutable: .\target\release\code-translator.exe" -ForegroundColor Cyan
    }
    
    "3" {
        Write-Host "▶️  Ejecutando aplicación..." -ForegroundColor Yellow
        cargo run
    }
    
    "4" {
        Write-Host "🧪 Ejecutando tests..." -ForegroundColor Yellow
        cargo test
        Write-Host "✅ Tests completados" -ForegroundColor Green
    }
    
    "5" {
        Write-Host "📖 Documentación disponible:" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "  • README.md              - Guía principal"
        Write-Host "  • INSTALL_RUST.md        - Instalación de Rust"
        Write-Host "  • PROJECT_STRUCTURE.md   - Estructura del proyecto"
        Write-Host "  • BUILD_GUIDE.md         - Guía de compilación mult-plataforma"
        Write-Host "  • API.md                 - Referencia de API"
        Write-Host "  • EXAMPLES.md            - Ejemplos de traducción"
        Write-Host ""
    }
    
    "6" {
        Write-Host "🧹 Limpiando compilaciones..." -ForegroundColor Yellow
        cargo clean
        Write-Host "✅ Limpieza completada" -ForegroundColor Green
    }
    
    default {
        Write-Host "❌ Opción no válida" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "Para más información, consulta README.md" -ForegroundColor Gray
