use std::io::{self, Write};
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
        println!("3. Salir");
        print!("Selecciona una opción: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => commands::translate_code(&mut translator),
            "2" => show_supported_languages(),
            "3" => {
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
