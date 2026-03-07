use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use std::thread;
use code_translator::{Language, cli::CodeTranslator};

mod commands;

fn main() {
    let language_count = Language::supported_languages().len();

    println!("╔═══════════════════════════════════════════╗");
    println!("║      CODE TRANSLATOR - Multi-Language     ║");
    println!("║ {} lenguajes + pseudolenguajes soportados ║", language_count);
    println!("╚═══════════════════════════════════════════╝\n");

    let mut translator = CodeTranslator::new();
    
    loop {
        println!("\n--- MENU ---");
        println!("1. Traducir código");
        println!("2. Ver lenguajes soportados");
        println!("3. Info");
        println!("0. Salir");
        print!("Selecciona una opción: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => commands::translate_code(&mut translator),
            "2" => show_supported_languages(),
            "3" => show_info_menu(),
            "0" => {
                println!("\n¡Hasta luego!");
                break;
            }
            _ => println!("Opción no válida. Intenta de nuevo."),
        }
    }
}

fn show_supported_languages() {
    println!("\n📋 LENGUAJES SOPORTADOS:");
    println!("  Entrada / Salida:");
    for lang in Language::supported_languages() {
        println!("    • {}", lang);
    }
}

fn show_info_menu() {
    // Mostrar contenido de Traductor_code_pseudo.rs
    show_traductor_info();
    
    println!("\n--- SUBMENU INFO ---");
    println!("0. Return to menu");
    println!("1. Info code lines");
    println!("\n⏳ Selección automática en 60 segundos...");
    
    let start_time = Instant::now();
    let timeout = Duration::from_secs(60);
    
    loop {
        print!("\nSelecciona una opción: ");
        io::stdout().flush().unwrap();
        
        // Verificar si ha pasado el timeout
        if start_time.elapsed() >= timeout {
            println!("\n⏱️ Tiempo agotado, mostrando info de líneas de código...");
            show_code_lines_info();
            return;
        }
        
        // Intentar leer con un pequeño delay para verificar timeout
        let mut choice = String::new();
        let stdin = io::stdin();
        
        // Crear un thread para leer input
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            tx.send(input).unwrap();
        });
        
        // Esperar input o timeout
        match rx.recv_timeout(timeout - start_time.elapsed()) {
            Ok(input) => {
                match input.trim() {
                    "0" => {
                        println!("Volviendo al menú principal...");
                        return;
                    }
                    "1" => {
                        show_code_lines_info();
                        return;
                    }
                    _ => println!("Opción no válida. Intenta de nuevo."),
                }
            }
            Err(_) => {
                println!("\n⏱️ Tiempo agotado, mostrando info de líneas de código...");
                show_code_lines_info();
                return;
            }
        }
    }
}

fn show_traductor_info() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    TRADUCTOR INFO                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let traductor_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Traductor_code_pseudo.rs");
    let content = fs::read_to_string(&traductor_path)
        .unwrap_or_else(|_| include_str!("../Traductor_code_pseudo.rs").to_string());

    for line in content.lines() {
        if line.trim().starts_with("println!") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start < end {
                        let text = &line[start + 1..end];
                        let text = text.replace("\\n", "\n");
                        println!("{}", text);
                    }
                }
            }
        }
    }
}

fn show_code_lines_info() {
    let project_root = ".";
    let total_lines = count_code_lines(project_root);
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          ESTADÍSTICAS DEL PROYECTO CODE TRANSLATOR       ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    println!("📊 Total de líneas de código en el proyecto: {} líneas\n", total_lines);
    
    println!("📁 Archivos analizados:");
    println!("   • Archivos Rust (.rs)");
    println!("   • Archivos C (.c, .h)");
    println!("   • Archivos de configuración (.toml, .yml, .gradle, .xml, .properties)");
    println!("   • Archivos de documentación (.md)");
    println!("   • Archivos Kotlin (.kt)");
    println!("   • Archivos de script (.sh, .ps1)\n");
    
    // Intentar compilar y ejecutar el programa C si existe
    #[cfg(target_os = "windows")]
    {
        compile_and_run_c_program(total_lines);
    }
    
    println!("\n⏳ Presiona Enter para continuar...");
    let mut wait = String::new();
    io::stdin().read_line(&mut wait).unwrap();
}

fn count_code_lines(path: &str) -> usize {
    let mut total = 0;
    let extensions = vec![
        "rs", "c", "h", "cpp", "hpp", "toml", "yml", "yaml", 
        "gradle", "xml", "md", "kt", "sh", "ps1", "properties"
    ];
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            
            // Ignorar directorios target, node_modules, .git, etc.
            if entry_path.is_dir() {
                let dir_name = entry_path.file_name().unwrap().to_str().unwrap();
                if dir_name != "target" && dir_name != "node_modules" && !dir_name.starts_with('.') {
                    total += count_code_lines(entry_path.to_str().unwrap());
                }
            } else if entry_path.is_file() {
                if let Some(ext) = entry_path.extension() {
                    if extensions.contains(&ext.to_str().unwrap()) {
                        if let Ok(content) = fs::read_to_string(&entry_path) {
                            total += content.lines().count();
                        }
                    }
                }
            }
        }
    }
    
    total
}

#[cfg(target_os = "windows")]
fn compile_and_run_c_program(total_lines: usize) {
    use std::process::Command;
    
    let c_file = "lineas_totales_proyecto.c";
    
    if !Path::new(c_file).exists() {
        return;
    }
    
    println!("🔨 Compilando programa C auxiliar...");
    
    // Crear un archivo con la variable externa
    let var_file = "total_lines_var.c";
    let var_content = format!("int total_code_lines = {};", total_lines);
    if fs::write(var_file, var_content).is_err() {
        return;
    }
    
    // Compilar con gcc si está disponible
    let output = Command::new("gcc")
        .args(&[c_file, var_file, "-o", "lineas_totales_proyecto.exe"])
        .output();
    
    if output.is_ok() && output.unwrap().status.success() {
        println!("✅ Compilación exitosa, ejecutando...\n");
        let _ = Command::new("lineas_totales_proyecto.exe").status();
        
        // Limpiar archivos temporales
        let _ = fs::remove_file("lineas_totales_proyecto.exe");
        let _ = fs::remove_file(var_file);
    } else {
        println!("⚠️  No se pudo compilar el programa C (gcc no disponible)");
    }
}

#[cfg(not(target_os = "windows"))]
fn compile_and_run_c_program(total_lines: usize) {
    use std::process::Command;
    
    let c_file = "lineas_totales_proyecto.c";
    
    if !Path::new(c_file).exists() {
        return;
    }
    
    println!("🔨 Compilando programa C auxiliar...");
    
    // Crear un archivo con la variable externa
    let var_file = "total_lines_var.c";
    let var_content = format!("int total_code_lines = {};", total_lines);
    if fs::write(var_file, var_content).is_err() {
        return;
    }
    
    // Compilar con gcc
    let output = Command::new("gcc")
        .args(&[c_file, var_file, "-o", "lineas_totales_proyecto"])
        .output();
    
    if output.is_ok() && output.unwrap().status.success() {
        println!("✅ Compilación exitosa, ejecutando...\n");
        let _ = Command::new("./lineas_totales_proyecto").status();
        
        // Limpiar archivos temporales
        let _ = fs::remove_file("lineas_totales_proyecto");
        let _ = fs::remove_file(var_file);
    } else {
        println!("⚠️  No se pudo compilar el programa C (gcc no disponible)");
    }
}
