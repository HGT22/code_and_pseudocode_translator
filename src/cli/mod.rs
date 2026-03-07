// CLI Module - User interface and translation orchestration
use crate::{core::lexer::Lexer, core::parser::Parser, core::transpiler::CodeGenerator, Language};
use regex::Regex;

pub struct CodeTranslator;

impl CodeTranslator {
    pub fn new() -> Self {
        CodeTranslator
    }

    pub fn translate(&self, code: &str, source_lang: Language, target_lang: Language) -> Result<String, String> {
        if code.trim().is_empty() {
            return Err("El código de entrada está vacío".to_string());
        }

        let normalized_code = self.normalize_source(code, source_lang);

        // Step 1: Lexical analysis
        let mut lexer = Lexer::new(&normalized_code);
        let tokens = lexer.tokenize();

        // Step 2: Parsing to AST
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Error de análisis: {}", e))?;

        // Step 3: Transformation (could add language-specific preprocessing here)
        // For now, we use the AST directly

        // Step 4: Code generation
        let generator = CodeGenerator::new(target_lang);
        let output_code = generator.generate(&ast);

        Ok(output_code)
    }

    fn normalize_source(&self, code: &str, source_lang: Language) -> String {
        let mut normalized = code.to_string();

        normalized = self.normalize_common_c_like(&normalized);

        match source_lang {
            Language::Python | Language::PseudocodePython | Language::Ruby => {
                normalized = normalized.replace("def ", "function ");
                normalized = normalized.replace("None", "null");
                normalized = normalized.replace("nil", "null");
                normalized = normalized.replace("True", "true");
                normalized = normalized.replace("False", "false");
                normalized = self.normalize_python_like_indentation(&normalized);
            }
            Language::Go => {
                normalized = normalized.replace("func ", "function ");
                normalized = normalized.replace(" nil", " null");
            }
            Language::Kotlin => {
                normalized = normalized.replace("fun ", "function ");
                normalized = normalized.replace("val ", "let ");
                normalized = normalized.replace("var ", "let ");
            }
            Language::Php => {
                normalized = normalized.replace("<?php", "");
                normalized = normalized.replace("$", "");
            }
            Language::R => {
                normalized = normalized.replace("<-", "=");
                normalized = normalized.replace("function", "function");
            }
            Language::Scala => {
                normalized = normalized.replace("def ", "function ");
                normalized = normalized.replace("val ", "let ");
                normalized = normalized.replace("var ", "let ");
            }
            Language::Dart => {
                normalized = normalized.replace("final ", "let ");
                normalized = normalized.replace("var ", "let ");
            }
            Language::Haskell => {
                normalized = normalized.replace("True", "true");
                normalized = normalized.replace("False", "false");
            }
            Language::Elixir => {
                normalized = normalized.replace("def ", "function ");
                normalized = normalized.replace("nil", "null");
            }
            Language::FSharp => {
                normalized = normalized.replace("let mutable ", "let ");
                normalized = normalized.replace("let ", "let ");
            }
            Language::Sql => {
                normalized = normalized.replace("TRUE", "true");
                normalized = normalized.replace("FALSE", "false");
                normalized = normalized.replace("NULL", "null");
            }
            Language::Matlab => {
                normalized = normalized.replace("%", "//");
            }
            Language::D => {
                normalized = normalized.replace("auto ", "let ");
            }
            Language::Assembly | Language::WebAssemblyWat => {
                normalized = normalized.replace(";", "//");
                normalized = normalized.replace(";;", "//");
            }
            Language::TypeScript => {
                normalized = normalized.replace(": number", "");
                normalized = normalized.replace(": string", "");
                normalized = normalized.replace(": boolean", "");
                normalized = normalized.replace(": any", "");
            }
            Language::Rust => {
                normalized = normalized.replace("fn ", "function ");
                normalized = normalized.replace("let mut ", "let ");
                normalized = normalized.replace("let ", "let ");
            }
            Language::Swift => {
                normalized = normalized.replace("func ", "function ");
                normalized = normalized.replace("var ", "let ");
            }
            Language::Java => {
                normalized = normalized.replace("boolean", "bool");
            }
            Language::CSharp => {
                normalized = normalized.replace("string", "str");
                normalized = normalized.replace("bool", "boolean");
            }
            Language::PseudocodeC | Language::PseudocodeJava => {
                normalized = normalized.replace("←", "=");
                normalized = normalized.replace("RETORNAR", "return");
                normalized = normalized.replace("retornar", "return");
                normalized = normalized.replace("LLAMAR", "");
                normalized = normalized.replace("CRIDAR", "");
                normalized = self.normalize_spanish_pseudocode(&normalized);
                normalized = self.normalize_catalan_pseudocode(&normalized);
                normalized = self.ensure_function_block_opening(&normalized);
            }
            _ => {}
        }

        normalized
    }

    fn normalize_common_c_like(&self, code: &str) -> String {
        let mut normalized = code.to_string();

        // Remove C/C++ preprocessor directives
        let preprocessor_re = Regex::new(r"(?m)^\s*#.*$").unwrap();
        normalized = preprocessor_re.replace_all(&normalized, "").to_string();

        // Fix C-style functions where { is on the next line
        let brace_newline_re = Regex::new(
            r"(?m)^(\s*(?:public\s+|private\s+|protected\s+|static\s+|final\s+|virtual\s+|override\s+|inline\s+|extern\s+|async\s+)*(?:void|int|float|double|char|bool|boolean|String|string|long|short|byte|auto|var|let|const|fn|def|func|fun)\s+[A-Za-z_][A-Za-z0-9_]*\s*\([^)]*\))\s*\n\s*\{"
        ).unwrap();
        normalized = brace_newline_re.replace_all(&normalized, "$1 {").to_string();

        let function_re = Regex::new(
            r"(?m)^\s*(?:public\s+|private\s+|protected\s+|static\s+|final\s+|virtual\s+|override\s+|inline\s+|extern\s+|async\s+)*(?:void|int|float|double|char|bool|boolean|String|string|long|short|byte|auto|var|let|const|fn|def|func|fun)\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*\{" 
        ).unwrap();

        normalized = function_re
            .replace_all(&normalized, |caps: &regex::Captures| {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("func");
                let params = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let stripped = self.strip_param_types(params);
                format!("function {}({}) {{", name, stripped)
            })
            .to_string();

        let typed_decl_re = Regex::new(
            r"(?m)^(\s*)(?:int|float|double|char|bool|boolean|String|string|long|short|byte|auto|var|val|let|const)\s+([A-Za-z_][A-Za-z0-9_]*)\s*="
        ).unwrap();
        normalized = typed_decl_re.replace_all(&normalized, "$1let $2 =").to_string();

        let typed_decl_no_init_re = Regex::new(
            r"(?m)^(\s*)(?:int|float|double|char|bool|boolean|String|string|long|short|byte|auto|var|val|let|const)\s+([A-Za-z_][A-Za-z0-9_]*)\s*;"
        ).unwrap();
        normalized = typed_decl_no_init_re
            .replace_all(&normalized, "$1let $2 = null;")
            .to_string();

        normalized
    }

    fn strip_param_types(&self, params: &str) -> String {
        if params.trim().is_empty() {
            return String::new();
        }

        params
            .split(',')
            .map(|part| {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    return String::new();
                }

                let without_default = trimmed.split('=').next().unwrap_or(trimmed).trim();
                let tokens: Vec<&str> = without_default
                    .split_whitespace()
                    .filter(|t| {
                        !matches!(
                            *t,
                            "int"
                                | "float"
                                | "double"
                                | "char"
                                | "bool"
                                | "boolean"
                                | "String"
                                | "string"
                                | "long"
                                | "short"
                                | "byte"
                                | "var"
                                | "let"
                                | "const"
                                | "final"
                                | "in"
                                | "out"
                                | "ref"
                        )
                    })
                    .collect();

                if let Some(last) = tokens.last() {
                    last.trim_matches(&['&', '*', '?', ':', '[', ']'][..]).to_string()
                } else {
                    without_default.to_string()
                }
            })
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn normalize_python_like_indentation(&self, code: &str) -> String {
        let mut output = String::new();
        let mut stack: Vec<usize> = vec![0];

        for raw_line in code.lines() {
            let line = raw_line.replace('\t', "    ");
            let trimmed_end = line.trim_end();
            if trimmed_end.trim().is_empty() {
                continue;
            }

            let indent = trimmed_end.chars().take_while(|c| *c == ' ').count();
            while stack.len() > 1 && indent < *stack.last().unwrap_or(&0) {
                stack.pop();
                output.push_str("}\n");
            }

            let mut statement = trimmed_end.trim().to_string();
            if statement.ends_with(':') {
                statement.pop();
                if let Some(rest) = statement.strip_prefix("def ") {
                    statement = format!("function {} {{", rest);
                } else if let Some(rest) = statement.strip_prefix("if ") {
                    statement = format!("if {} {{", rest);
                } else if let Some(rest) = statement.strip_prefix("elif ") {
                    output.push_str("} else ");
                    statement = format!("if {} {{", rest);
                } else if statement == "else" {
                    output.push_str("} else {\n");
                    stack.push(indent + 4);
                    continue;
                } else if let Some(rest) = statement.strip_prefix("while ") {
                    statement = format!("while {} {{", rest);
                } else if let Some(rest) = statement.strip_prefix("for ") {
                    statement = format!("for {} {{", rest);
                } else {
                    statement.push_str(" {");
                }
                output.push_str(&statement);
                output.push('\n');
                stack.push(indent + 4);
                continue;
            }

            if !statement.ends_with(';') && !statement.ends_with('{') && !statement.ends_with('}') {
                statement.push(';');
            }
            output.push_str(&statement);
            output.push('\n');
        }

        while stack.len() > 1 {
            stack.pop();
            output.push_str("}\n");
        }

        output
    }

    fn normalize_spanish_pseudocode(&self, code: &str) -> String {
        let mut normalized = code.to_string();

        normalized = normalized.replace("FUNCIÓN", "function");
        normalized = normalized.replace("FUNCION", "function");
        normalized = normalized.replace("MÉTODO", "function");
        normalized = normalized.replace("METODO", "function");
        normalized = normalized.replace("SI ", "if ");
        normalized = normalized.replace(" ENTONCES", " {");
        normalized = normalized.replace("SINO", "} else {");
        normalized = normalized.replace("FIN SI", "}");
        normalized = normalized.replace("MIENTRAS ", "while ");
        normalized = normalized.replace(" HACER", " {");
        normalized = normalized.replace("FIN MIENTRAS", "}");
        normalized = normalized.replace("PARA ", "for ");
        normalized = normalized.replace(" EN ", " in ");
        normalized = normalized.replace("FIN PARA", "}");
        normalized = normalized.replace("FIN FUNCIÓN", "}");
        normalized = normalized.replace("FIN FUNCION", "}");
        normalized = normalized.replace("FIN MÉTODO", "}");
        normalized = normalized.replace("FIN METODO", "}");

        normalized
    }

    fn normalize_catalan_pseudocode(&self, code: &str) -> String {
        let mut normalized = code.to_string();

        normalized = normalized.replace("FUNCIÓ", "function");
        normalized = normalized.replace("FUNCIO", "function");
        normalized = normalized.replace("funció", "function");
        normalized = normalized.replace("funcio", "function");
        normalized = normalized.replace("MÈTODE", "function");
        normalized = normalized.replace("METODE", "function");
        normalized = normalized.replace("mètode", "function");
        normalized = normalized.replace("metode", "function");
        normalized = normalized.replace("RETORNAR", "return");
        normalized = normalized.replace("retornar", "return");
        normalized = normalized.replace("SI ", "if ");
        normalized = normalized.replace("si ", "if ");
        normalized = normalized.replace(" ALESHORES", " {");
        normalized = normalized.replace(" LLAVORS", " {");
        normalized = normalized.replace(" aleshores", " {");
        normalized = normalized.replace(" llavors", " {");
        normalized = normalized.replace("SINÓ", "} else {");
        normalized = normalized.replace("SINO", "} else {");
        normalized = normalized.replace("sinó", "} else {");
        normalized = normalized.replace("sino", "} else {");
        normalized = normalized.replace("FI SI", "}");
        normalized = normalized.replace("fi si", "}");
        normalized = normalized.replace("MENTRE ", "while ");
        normalized = normalized.replace("mentre ", "while ");
        normalized = normalized.replace(" FER", " {");
        normalized = normalized.replace(" fer", " {");
        normalized = normalized.replace("FI MENTRE", "}");
        normalized = normalized.replace("fi mentre", "}");
        normalized = normalized.replace("PER ", "for ");
        normalized = normalized.replace("per ", "for ");
        normalized = normalized.replace(" EN ", " in ");
        normalized = normalized.replace(" en ", " in ");
        normalized = normalized.replace("FI PER", "}");
        normalized = normalized.replace("fi per", "}");
        normalized = normalized.replace("FI FUNCIÓ", "}");
        normalized = normalized.replace("FI FUNCIO", "}");
        normalized = normalized.replace("FI MÈTODE", "}");
        normalized = normalized.replace("FI METODE", "}");
        normalized = normalized.replace("fi funció", "}");
        normalized = normalized.replace("fi funcio", "}");
        normalized = normalized.replace("fi mètode", "}");
        normalized = normalized.replace("fi metode", "}");

        normalized
    }

    fn ensure_function_block_opening(&self, code: &str) -> String {
        let fn_without_block = Regex::new(r"(?m)^\s*function\s+[A-Za-z_][A-Za-z0-9_]*\s*\([^)]*\)\s*$").unwrap();
        fn_without_block
            .replace_all(code, |caps: &regex::Captures| {
                let line = caps.get(0).map(|m| m.as_str()).unwrap_or("").trim_end();
                format!("{} {{", line)
            })
            .to_string()
    }

    pub fn translate_file(&self, file_path: &str, source_lang: Language, target_lang: Language) -> Result<String, String> {
        let code = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Error al leer archivo: {}", e))?;
        self.translate(&code, source_lang, target_lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Language;

    #[test]
    fn test_translate_catalan_pseudocode_c_to_c() {
        let translator = CodeTranslator::new();
        let input = "FUNCIÓ suma(a, b)\nRETORNAR a + b\nFI FUNCIÓ\n";

        let output = translator
            .translate(input, Language::PseudocodeC, Language::C)
            .expect("translation should succeed");

        assert!(output.contains("int suma") || output.contains("suma("));
        assert!(output.contains("return (a + b)") || output.contains("return a + b"));
    }

    #[test]
    fn test_translate_spanish_pseudocode_c_to_python() {
        let translator = CodeTranslator::new();
        let input = "FUNCIÓN suma(a, b)\nRETORNAR a + b\nFIN FUNCIÓN\n";

        let output = translator
            .translate(input, Language::PseudocodeC, Language::Python)
            .expect("translation should succeed");

        assert!(output.contains("def suma(a, b):"));
        assert!(output.contains("return (a + b)") || output.contains("return a + b"));
    }

    #[test]
    fn test_translate_java_to_typescript_basic() {
        let translator = CodeTranslator::new();
        let input = "int x = 10;\n";

        let output = translator
            .translate(input, Language::Java, Language::TypeScript)
            .expect("translation should succeed");

        assert!(output.contains("let x = 10;"));
    }

    #[test]
    fn test_translate_go_to_rust_basic() {
        let translator = CodeTranslator::new();
        let input = "var x = 5;\n";

        let output = translator
            .translate(input, Language::Go, Language::Rust)
            .expect("translation should succeed");

        assert!(output.contains("let mut x = 5;"));
    }
}
