use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use std::thread;
use code_translator::{Language, cli::CodeTranslator};

mod commands;

fn main() {
    let language_count = Language::supported_languages().len();

    println!("â•”â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•—");
    println!("â•‘      CODE TRANSLATOR - Multi-Language     â•‘");
    println!("â•‘ {} lenguajes + pseudolenguajes soportados â•‘", language_count);
    println!("â•šâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n");

    let mut translator = CodeTranslator::new();
    
    loop {
        println!("\n--- MENU ---");
        println!("1. Traducir cÃ³digo");
        println!("2. Ver lenguajes soportados");
        println!("3. Info");
        println!("0. Salir");
        print!("Selecciona una opciÃ³n: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => commands::translate_code(&mut translator),
            "2" => show_supported_languages(),
            "3" => show_info_menu(),
            "0" => {
                println!("\nÂ¡Hasta luego!");
                break;
            }
            _ => println!("OpciÃ³n no vÃ¡lida. Intenta de nuevo."),
        }
    }
}

fn show_supported_languages() {
    println!("\nðŸ“‹ LENGUAJES SOPORTADOS:");
    println!("  Entrada / Salida:");
    for lang in Language::supported_languages() {
        println!("    â€¢ {}", lang);
    }
}

fn show_info_menu() {
    // Mostrar contenido de Traductor_code_pseudo.rs
    show_traductor_info();
    
    println!("\n--- SUBMENU INFO ---");
    println!("0. Return to menu");
    println!("1. Info code lines");
    println!("\nâ³ SelecciÃ³n automÃ¡tica en 60 segundos...");
    
    let start_time = Instant::now();
    let timeout = Duration::from_secs(60);
    
    loop {
        print!("\nSelecciona una opciÃ³n: ");
        io::stdout().flush().unwrap();
        
        // Verificar si ha pasado el timeout
        if start_time.elapsed() >= timeout {
            println!("\nâ±ï¸ Tiempo agotado, mostrando info de lÃ­neas de cÃ³digo...");
            show_code_lines_info();
            return;
        }
        
        // Intentar leer con un pequeÃ±o delay para verificar timeout
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
                        println!("Volviendo al menÃº principal...");
                        return;
                    }
                    "1" => {
                        show_code_lines_info();
                        return;
                    }
                    _ => println!("OpciÃ³n no vÃ¡lida. Intenta de nuevo."),
                }
            }
            Err(_) => {
                println!("\nâ±ï¸ Tiempo agotado, mostrando info de lÃ­neas de cÃ³digo...");
                show_code_lines_info();
                return;
            }
        }
    }
}

fn show_traductor_info() {
    println!("\n");
    println!("                    TRADUCTOR INFO                         ");
    println!("\n");
    println!("Hello World!");
    println!("This is a pseudo-code translator.");
    println!("It can translate code between multiple programming languages and pseudocode formats.");
    println!("Supported languages include C, C++, Python, Java, JavaScript, Rust, Swift, and more.");
    println!("You can input code in one language and get the equivalent code in another language.");
    println!("The translator normalizes syntax and constructs to ensure accurate translations.");
    println!("Try it out and see how it works!");
    println!("Note: This is a simplified demonstration and may not cover all edge cases or language features.");
    println!("Happy coding!");
    println!(" ");
    println!(" ");
    println!("Socialmedia:");
    println!("GitHub: https://github.com/HGT22");
    println!("LinkedIn: https://www.linkedin.com/in/héctor-garcía-de-la-torre-132296388?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app");
    println!("Twitter: https://twitter.com/hgtdev22");
    println!("Instagram: https://www.instagram.com/hgtdev22/");
    println!("TikTok: https://www.tiktok.com/@hgtdev22");
    println!("YouTube: https://www.youtube.com/@hgtdev22");
    println!(" ");
    println!("Contact:");
    println!("Email: hgarcia2008a@gmail.com");
    println!(" ");
    println!("Thank you for using the pseudo-code translator!");
    println!("Feel free to contribute or report issues on GitHub.");
    println!("Goodbye!");
    println!(" ");
    println!(" ");
}

fn show_code_lines_info() {
    let project_root = ".";
    let total_lines = count_code_lines(project_root);
    
    println!("\nâ•”â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•—");
    println!("â•‘          ESTADÃSTICAS DEL PROYECTO CODE TRANSLATOR       â•‘");
    println!("â•šâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n");
    
    println!("ðŸ“Š Total de lÃ­neas de cÃ³digo en el proyecto: {} lÃ­neas\n", total_lines);
    
    println!("ðŸ“ Archivos analizados:");
    println!("   â€¢ Archivos Rust (.rs)");
    println!("   â€¢ Archivos C (.c, .h)");
    println!("   â€¢ Archivos de configuraciÃ³n (.toml, .yml, .gradle, .xml, .properties)");
    println!("   â€¢ Archivos de documentaciÃ³n (.md)");
    println!("   â€¢ Archivos Kotlin (.kt)");
    println!("   â€¢ Archivos de script (.sh, .ps1)\n");
    
    // Intentar compilar y ejecutar el programa C si existe
    #[cfg(target_os = "windows")]
    {
        compile_and_run_c_program(total_lines);
    }
    
    println!("\nâ³ Presiona Enter para continuar...");
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
    
    println!("ðŸ”¨ Compilando programa C auxiliar...");
    
    // Crear un archivo con la variable externa
    let var_file = "total_lines_var.c";
    let var_content = format!("int total_code_lines = {};", total_lines);
    if fs::write(var_file, var_content).is_err() {
        return;
    }
    
    // Compilar con gcc si estÃ¡ disponible
    let output = Command::new("gcc")
        .args(&[c_file, var_file, "-o", "lineas_totales_proyecto.exe"])
        .output();
    
    if output.is_ok() && output.unwrap().status.success() {
        println!("âœ… CompilaciÃ³n exitosa, ejecutando...\n");
        let _ = Command::new("lineas_totales_proyecto.exe").status();
        
        // Limpiar archivos temporales
        let _ = fs::remove_file("lineas_totales_proyecto.exe");
        let _ = fs::remove_file(var_file);
    } else {
        println!("âš ï¸  No se pudo compilar el programa C (gcc no disponible)");
    }
}

#[cfg(not(target_os = "windows"))]
fn compile_and_run_c_program(total_lines: usize) {
    use std::process::Command;
    
    let c_file = "lineas_totales_proyecto.c";
    
    if !Path::new(c_file).exists() {
        return;
    }
    
    println!("ðŸ”¨ Compilando programa C auxiliar...");
    
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
        println!("âœ… CompilaciÃ³n exitosa, ejecutando...\n");
        let _ = Command::new("./lineas_totales_proyecto").status();
        
        // Limpiar archivos temporales
        let _ = fs::remove_file("lineas_totales_proyecto");
        let _ = fs::remove_file(var_file);
    } else {
        println!("âš ï¸  No se pudo compilar el programa C (gcc no disponible)");
    }
}
