use code_translator::{core::ast::ASTNode, core::transpiler::generate_code, cli::CodeTranslator, Language};

#[test]
fn bateria_generacion_todos_lenguajes_no_vacia() {
    let ast = ASTNode::Program(vec![
        Box::new(ASTNode::VarDeclaration {
            name: "x".to_string(),
            data_type: code_translator::core::ast::DataType::Integer,
            value: Some(Box::new(ASTNode::IntegerLiteral(7))),
        }),
        Box::new(ASTNode::ReturnStatement(Some(Box::new(ASTNode::Identifier(
            "x".to_string(),
        ))))),
    ]);

    let languages = [
        Language::C,
        Language::Cpp,
        Language::CSharp,
        Language::ObjectiveC,
        Language::ObjectiveCpp,
        Language::Python,
        Language::Java,
        Language::JavaScript,
        Language::TypeScript,
        Language::Rust,
        Language::Swift,
        Language::Ruby,
        Language::Go,
        Language::Kotlin,
        Language::Php,
        Language::R,
        Language::Scala,
        Language::Dart,
        Language::Haskell,
        Language::Elixir,
        Language::FSharp,
        Language::Sql,
        Language::Matlab,
        Language::D,
        Language::Assembly,
        Language::WebAssemblyWat,
        Language::PseudocodeC,
        Language::PseudocodeJava,
        Language::PseudocodePython,
    ];

    for language in languages {
        let out = generate_code(&ast, language);
        assert!(!out.trim().is_empty(), "Salida vacía para {:?}", language);
    }
}

#[test]
fn bateria_aliases_catalan_pseudocodi() {
    assert_eq!(Language::from_string("pseudocodi"), Some(Language::PseudocodeC));
    assert_eq!(
        Language::from_string("pseudocodi orientat a c"),
        Some(Language::PseudocodeC)
    );
    assert_eq!(
        Language::from_string("pseudocodi orientat a java"),
        Some(Language::PseudocodeJava)
    );
    assert_eq!(
        Language::from_string("pseudocodi orientat a python"),
        Some(Language::PseudocodePython)
    );
}

#[test]
fn bateria_traduccion_pseudocodi_cat_urv() {
    let translator = CodeTranslator::new();
    let src = "FUNCIÓ suma(a, b)\nSI a > b ALESHORES\nRETORNAR a\nSINÓ\nRETORNAR b\nFI SI\nFI FUNCIÓ\n";

    let out_c = translator
        .translate(src, Language::PseudocodeC, Language::C)
        .expect("Traducción a C debería funcionar");
    assert!(out_c.contains("suma("));
    assert!(out_c.contains("if ("));

    let out_pseudocodi = translator
        .translate(src, Language::PseudocodeC, Language::PseudocodeC)
        .expect("Traducción a pseudocodi C debería funcionar");
    assert!(out_pseudocodi.contains("PSEUDOCODI ORIENTAT A C"));
}

#[test]
fn bateria_traduccion_basica_cross_language() {
    let translator = CodeTranslator::new();
    let src = "int x = 10;";

    let out_ts = translator
        .translate(src, Language::C, Language::TypeScript)
        .expect("C -> TS debería funcionar");
    assert!(out_ts.contains("let x = 10;"));

    let out_go = translator
        .translate(src, Language::C, Language::Go)
        .expect("C -> Go debería funcionar");
    assert!(out_go.contains("var x = 10;"));
}
