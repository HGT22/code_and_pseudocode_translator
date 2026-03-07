// Library root - Exposes public API
pub mod core;
pub mod cli;

// Android FFI
#[cfg(target_os = "android")]
pub mod android_ffi;

// iOS FFI
#[cfg(target_os = "ios")]
pub mod ios_ffi;

pub use core::{ast, lexer, parser, transpiler};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    C,
    Cpp,
    CSharp,
    ObjectiveC,
    ObjectiveCpp,
    Python,
    Java,
    JavaScript,
    TypeScript,
    Rust,
    Swift,
    Ruby,
    Go,
    Kotlin,
    Php,
    R,
    Scala,
    Dart,
    Haskell,
    Elixir,
    FSharp,
    Sql,
    Matlab,
    D,
    Assembly,
    WebAssemblyWat,
    PseudocodeC,
    PseudocodeJava,
    PseudocodePython,
}

impl Language {
    pub fn from_string(s: &str) -> Option<Self> {
        let normalized = Self::normalize_language_input(s);

        match normalized.as_str() {
            "c" => Some(Language::C),
            "c++" | "cpp" => Some(Language::Cpp),
            "c#" | "csharp" | "cs" => Some(Language::CSharp),
            "objective-c" | "objectivec" | "objc" => Some(Language::ObjectiveC),
            "objective-c++" | "objectivec++" | "objc++" | "objectivecpp" | "objective-cpp" => Some(Language::ObjectiveCpp),
            "python" | "py" => Some(Language::Python),
            "java" => Some(Language::Java),
            "javascript" | "js" => Some(Language::JavaScript),
            "typescript" | "ts" => Some(Language::TypeScript),
            "rust" | "rs" => Some(Language::Rust),
            "swift" => Some(Language::Swift),
            "ruby" | "rb" => Some(Language::Ruby),
            "go" | "golang" => Some(Language::Go),
            "kotlin" | "kt" => Some(Language::Kotlin),
            "php" => Some(Language::Php),
            "r" | "rlang" => Some(Language::R),
            "scala" => Some(Language::Scala),
            "dart" => Some(Language::Dart),
            "haskell" | "hs" => Some(Language::Haskell),
            "elixir" | "ex" | "exs" => Some(Language::Elixir),
            "f#" | "fsharp" | "fs" => Some(Language::FSharp),
            "sql" => Some(Language::Sql),
            "matlab" => Some(Language::Matlab),
            "d" | "dlang" => Some(Language::D),
            "assembly" | "assemblyx86x64arm" | "asm" | "x86" | "x64" | "arm" | "armasm" => {
                Some(Language::Assembly)
            }
            "webassembly" | "webassemblywat" | "wasm" | "wat" => Some(Language::WebAssemblyWat),
            "pseudocode"
            | "pseudo"
            | "pseudoc"
            | "pseudocodigo"
            | "pseudocodigoorientadoac"
            | "pseudocorientadoac"
            | "pseudoorientadoac"
            | "pseudocodec"
            | "pseudo-c"
            | "pseudoc-c"
            | "pseudocodi"
            | "pseudocodiorientatac"
            | "pseudocorientatac"
            | "pseudoorientatac"
            | "pseudocatic"
            | "pseudocodic"
            | "pseudocodiorientatc"
            | "pseudocodiorientatalc" => Some(Language::PseudocodeC),
            "pseudojava"
            | "pseudocodejava"
            | "pseudocodigoorientadoajava"
            | "pseudocjava"
            | "pseudojavaorientado"
            | "pseudocorientadoajava"
            | "pseudoorientadoajava"
            | "pseudo-java"
            | "pseudoc-java"
            | "pseudocodiorientatajava"
            | "pseudocorientatajava"
            | "pseudoorientatajava" => Some(Language::PseudocodeJava),
            "pseudopython"
            | "pseudocodepython"
            | "pseudocodigoorientadoapython"
            | "pseudocpython"
            | "pseudocorientadoapython"
            | "pseudoorientadoapython"
            | "pseudo-python"
            | "pseudoc-python"
            | "pseudocodiorientatapython"
            | "pseudocorientatapython"
            | "pseudoorientatapython" => Some(Language::PseudocodePython),
            _ => None,
        }
    }

    fn normalize_language_input(input: &str) -> String {
        input
            .to_lowercase()
            .replace(['á', 'à', 'ä', 'â'], "a")
            .replace(['é', 'è', 'ë', 'ê'], "e")
            .replace(['í', 'ì', 'ï', 'î'], "i")
            .replace(['ó', 'ò', 'ö', 'ô'], "o")
            .replace(['ú', 'ù', 'ü', 'û'], "u")
            .replace('ñ', "n")
                .replace('ç', "c")
                .replace('·', "")
            .replace([' ', '_', '/', '(', ')'], "")
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Language::C => "C",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::ObjectiveC => "Objective-C",
            Language::ObjectiveCpp => "Objective-C++",
            Language::Python => "Python",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Rust => "Rust",
            Language::Swift => "Swift",
            Language::Ruby => "Ruby",
            Language::Go => "Go",
            Language::Kotlin => "Kotlin",
            Language::Php => "PHP",
            Language::R => "R",
            Language::Scala => "Scala",
            Language::Dart => "Dart",
            Language::Haskell => "Haskell",
            Language::Elixir => "Elixir",
            Language::FSharp => "F#",
            Language::Sql => "SQL",
            Language::Matlab => "MATLAB",
            Language::D => "D",
            Language::Assembly => "Assembly (x86/x64/ARM)",
            Language::WebAssemblyWat => "WebAssembly (WAT)",
            Language::PseudocodeC => "Pseudocódigo orientado a C",
            Language::PseudocodeJava => "Pseudocódigo orientado a Java",
            Language::PseudocodePython => "Pseudocódigo orientado a Python",
        }
    }

    pub fn supported_languages() -> Vec<&'static str> {
        vec![
            "C",
            "C++",
            "C#",
            "Objective-C",
            "Objective-C++",
            "Python",
            "Java",
            "JavaScript",
            "TypeScript",
            "Rust",
            "Swift",
            "Ruby",
            "Go",
            "Kotlin",
            "PHP",
            "R",
            "Scala",
            "Dart",
            "Haskell",
            "Elixir",
            "F#",
            "SQL",
            "MATLAB",
            "D",
            "Assembly (x86/x64/ARM)",
            "WebAssembly (WAT)",
            "Pseudocódigo orientado a C",
            "Pseudocódigo orientado a Java",
            "Pseudocódigo orientado a Python",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_string() {
        assert_eq!(Language::from_string("c"), Some(Language::C));
        assert_eq!(Language::from_string("cpp"), Some(Language::Cpp));
        assert_eq!(Language::from_string("cs"), Some(Language::CSharp));
        assert_eq!(Language::from_string("python"), Some(Language::Python));
        assert_eq!(Language::from_string("java"), Some(Language::Java));
        assert_eq!(Language::from_string("js"), Some(Language::JavaScript));
        assert_eq!(Language::from_string("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_string("rust"), Some(Language::Rust));
        assert_eq!(Language::from_string("swift"), Some(Language::Swift));
        assert_eq!(Language::from_string("ruby"), Some(Language::Ruby));
        assert_eq!(Language::from_string("golang"), Some(Language::Go));
        assert_eq!(Language::from_string("kotlin"), Some(Language::Kotlin));
        assert_eq!(Language::from_string("php"), Some(Language::Php));
        assert_eq!(Language::from_string("r"), Some(Language::R));
        assert_eq!(Language::from_string("scala"), Some(Language::Scala));
        assert_eq!(Language::from_string("dart"), Some(Language::Dart));
        assert_eq!(Language::from_string("haskell"), Some(Language::Haskell));
        assert_eq!(Language::from_string("elixir"), Some(Language::Elixir));
        assert_eq!(Language::from_string("fsharp"), Some(Language::FSharp));
        assert_eq!(Language::from_string("sql"), Some(Language::Sql));
        assert_eq!(Language::from_string("matlab"), Some(Language::Matlab));
        assert_eq!(Language::from_string("dlang"), Some(Language::D));
        assert_eq!(Language::from_string("asm"), Some(Language::Assembly));
        assert_eq!(Language::from_string("wat"), Some(Language::WebAssemblyWat));
        assert_eq!(Language::from_string("pseudocode"), Some(Language::PseudocodeC));
        assert_eq!(Language::from_string("pseudocodi orientat a c"), Some(Language::PseudocodeC));
        assert_eq!(Language::from_string("pseudocodi"), Some(Language::PseudocodeC));
        assert_eq!(Language::from_string("pseudojava"), Some(Language::PseudocodeJava));
        assert_eq!(Language::from_string("pseudocodi orientat a java"), Some(Language::PseudocodeJava));
        assert_eq!(Language::from_string("pseudopython"), Some(Language::PseudocodePython));
        assert_eq!(Language::from_string("pseudocodi orientat a python"), Some(Language::PseudocodePython));
        assert_eq!(
            Language::from_string("pseudocódigo orientado a Java"),
            Some(Language::PseudocodeJava)
        );
        assert_eq!(
            Language::from_string("pseudocodigo orientado a Python"),
            Some(Language::PseudocodePython)
        );
        assert_eq!(
            Language::from_string("Pseudocódigo orientado a C"),
            Some(Language::PseudocodeC)
        );
        assert_eq!(
            Language::from_string("Assembly (x86/x64/ARM)"),
            Some(Language::Assembly)
        );
    }
}
