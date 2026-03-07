use crate::cli::CodeTranslator;
use crate::Language;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn ct_translate(
    source_code: *const c_char,
    source_language: *const c_char,
    target_language: *const c_char,
) -> *mut c_char {
    if source_code.is_null() || source_language.is_null() || target_language.is_null() {
        let error = CString::new("Invalid null pointer input").unwrap();
        return error.into_raw();
    }

    let source_code = unsafe { CStr::from_ptr(source_code) }
        .to_string_lossy()
        .into_owned();
    let source_language = unsafe { CStr::from_ptr(source_language) }
        .to_string_lossy()
        .into_owned();
    let target_language = unsafe { CStr::from_ptr(target_language) }
        .to_string_lossy()
        .into_owned();

    let source_lang = match Language::from_string(&source_language) {
        Some(lang) => lang,
        None => {
            let error = CString::new("Invalid source language").unwrap();
            return error.into_raw();
        }
    };

    let target_lang = match Language::from_string(&target_language) {
        Some(lang) => lang,
        None => {
            let error = CString::new("Invalid target language").unwrap();
            return error.into_raw();
        }
    };

    let mut translator = CodeTranslator::new();
    let output = match translator.translate(&source_code, source_lang, target_lang) {
        Ok(result) => result,
        Err(err) => format!("Translation error: {err}"),
    };

    CString::new(output)
        .unwrap_or_else(|_| CString::new("Encoding error").unwrap())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn ct_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}
