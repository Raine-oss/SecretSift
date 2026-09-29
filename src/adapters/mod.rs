pub mod java;
pub mod js_ts;
pub mod python;
pub mod rust;

use crate::models::{Language, SecretMatch};
use java::JavaAdapter;
use js_ts::JsTsAdapter;
use python::PythonAdapter;
use rust::RustAdapter;

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
        Language::JavaScript | Language::TypeScript => Some(Box::new(JsTsAdapter::new(lang))),
        Language::Python => Some(Box::new(PythonAdapter::new())),
        Language::Unknown => None,
    }
}
