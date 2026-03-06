use std::io::{self, Write};
use code_translator::{Language, cli::CodeTranslator};

pub fn translate_code(translator: &mut CodeTranslator) {
    println!("\n--- TRADUCTOR DE CÓDIGO ---\n");
    println!("Lenguajes disponibles:");
    for lang in Language::supported_languages() {
        println!("  - {}", lang);
    }
    println!();

    // Get source language
    print!("Lenguaje de entrada: ");
    io::stdout().flush().unwrap();
    let mut source_lang = String::new();
    io::stdin().read_line(&mut source_lang).unwrap();
    
    let source_lang = match Language::from_string(source_lang.trim()) {
        Some(lang) => lang,
        None => {
            println!("❌ Lenguaje de entrada no reconocido");
            return;
        }
    };

    // Get target language
    print!("Lenguaje de salida: ");
    io::stdout().flush().unwrap();
    let mut target_lang = String::new();
    io::stdin().read_line(&mut target_lang).unwrap();
    
    let target_lang = match Language::from_string(target_lang.trim()) {
        Some(lang) => lang,
        None => {
            println!("❌ Lenguaje de salida no reconocido");
            return;
        }
    };

    if source_lang == target_lang {
        println!("⚠️  Los lenguajes de entrada y salida son iguales.");
    }

    // Get code input
    println!("\nIngresa el código (escribe 'FIN' en una línea para terminar):");
    println!("─────────────────────────────────────────");
    
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "FIN" {
            break;
        }
        code.push_str(&line);
    }

    // Translate
    println!("\n⏳ Traduciendo...");
    match translator.translate(&code, source_lang, target_lang) {
        Ok(translated) => {
            println!("\n✅ CÓDIGO TRADUCIDO:");
            println!("─────────────────────────────────────────");
            println!("{}", translated);
            println!("─────────────────────────────────────────");
        }
        Err(e) => {
            println!("❌ Error en la traducción: {}", e);
        }
    }
}
