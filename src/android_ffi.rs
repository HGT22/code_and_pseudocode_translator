use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use crate::cli::CodeTranslator;
use crate::Language;

#[no_mangle]
pub extern "C" fn Java_com_urv_codetranslator_MainActivity_translateCode(
    mut env: JNIEnv,
    _class: JClass,
    source_code: JString,
    source_language: JString,
    target_language: JString,
) -> jstring {
    // Convert Java strings to Rust strings
    let source_code_str: String = match env.get_string(&source_code) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: invalid source code").unwrap().into_raw(),
    };

    let source_lang_str: String = match env.get_string(&source_language) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: invalid source language").unwrap().into_raw(),
    };

    let target_lang_str: String = match env.get_string(&target_language) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: invalid target language").unwrap().into_raw(),
    };

    // Parse languages
    let source_lang = match Language::from_string(&source_lang_str) {
        Some(lang) => lang,
        None => return env.new_string("Error parsing source language").unwrap().into_raw(),
    };

    let target_lang = match Language::from_string(&target_lang_str) {
        Some(lang) => lang,
        None => return env.new_string("Error parsing target language").unwrap().into_raw(),
    };

    // Translate
    let mut translator = CodeTranslator::new();
    match translator.translate(&source_code_str, source_lang, target_lang) {
        Ok(result) => env.new_string(result).unwrap().into_raw(),
        Err(e) => env.new_string(format!("Translation error: {}", e)).unwrap().into_raw(),
    }
}
