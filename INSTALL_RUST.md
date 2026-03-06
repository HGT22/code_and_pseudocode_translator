# INSTALACIÓN DE RUST - Guía para Windows

## Método 1: rustup.rs (RECOMENDADO)

### Paso 1: Descargar rustup
1. Ve a https://rustup.rs/
2. Haz clic en el botón de descargar (descargará `rustup-init.exe`)

### Paso 2: Instalar
1. Ejecuta `rustup-init.exe`
2. Presiona `1` (opción por defecto) y Enter
3. Espera a que se complete la instalación

### Paso 3: Reiniciar PowerShell
```powershell
# Cierra PowerShell completamente y abrelo nuevamente
refreshenv
```

### Paso 4: Verificar instalación
```powershell
rustc --version
cargo --version
```

## Método 2: Usar WSL (Windows Subsystem for Linux)

Si tienes WSL2 instalado:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Después de instalar Rust

### Construir el proyecto
```powershell
cd "C:\Users\Usuari\OneDrive\Escritorio\URV\2025-26\Programming\Fundamentos de Programacion\Fundamentos de  Programacion II\Code_Translator\GitHubHGT\Code Translator"

# Descargar dependencias
cargo build

# Compilación de release (optimizada)
cargo build --release
```

### Ejecutar el programa
```powershell
# En modo desarrollo
cargo run

# O ejecutar el binario directamente
.\target\release\code-translator.exe
```

## Problemas Comunes

### "cargo: command not found"
- Reinicia PowerShell completamente y vuelve a intentar

### Error de compilación
- Ejecuta `cargo clean` y luego `cargo build` nuevamente

### Permisos denegados
- Ejecuta PowerShell como administrador

## Verificación de la instalación

Una vez instalado, puedes verificar:
```powershell
rustup show
rustup update
```

---

¡Una vez completado, el proyecto debería compilar sin problemas!
