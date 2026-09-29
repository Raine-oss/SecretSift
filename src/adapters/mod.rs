pub mod cpp;
pub mod csharp;
pub mod dart;
pub mod go;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod swift;
pub mod typescript;

use crate::models::{Language, SecretMatch};
use cpp::CppAdapter;
use csharp::CSharpAdapter;
use dart::DartAdapter;
use go::GoAdapter;
use java::JavaAdapter;
use javascript::JavaScriptAdapter;
use kotlin::KotlinAdapter;
use php::PhpAdapter;
use python::PythonAdapter;
use ruby::RubyAdapter;
use rust::RustAdapter;
use swift::SwiftAdapter;
use typescript::TypeScriptAdapter;

// Language Adapter Trait

pub trait LanguageAdapter: Send + Sync {
    #[allow(dead_code)]
    fn language(&self) -> Language;
    fn rewrite_line(&self, match_info: &SecretMatch, line: &str) -> Option<String>;
    fn post_process_file_content(&self, content: &str) -> String {
        content.to_string()
    }
}

// Adapter Router

pub fn get_adapter_for_language(lang: Language) -> Option<Box<dyn LanguageAdapter>> {
    match lang {
        Language::Rust => Some(Box::new(RustAdapter::new())),
        Language::Java => Some(Box::new(JavaAdapter::new())),
        Language::JavaScript => Some(Box::new(JavaScriptAdapter::new())),
        Language::TypeScript => Some(Box::new(TypeScriptAdapter::new())),
        Language::Python => Some(Box::new(PythonAdapter::new())),
        Language::Go => Some(Box::new(GoAdapter::new())),
        Language::CSharp => Some(Box::new(CSharpAdapter::new())),
        Language::Kotlin => Some(Box::new(KotlinAdapter::new())),
        Language::Php => Some(Box::new(PhpAdapter::new())),
        Language::Ruby => Some(Box::new(RubyAdapter::new())),
        Language::C => Some(Box::new(CppAdapter::new(Language::C))),
        Language::Cpp => Some(Box::new(CppAdapter::new(Language::Cpp))),
        Language::Dart => Some(Box::new(DartAdapter::new())),
        Language::Swift => Some(Box::new(SwiftAdapter::new())),
        Language::Unknown => None,
    }
}
