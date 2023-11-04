#[cfg(feature = "en")]
pub mod scanner;

pub mod error;
pub mod token;

pub mod tokenizer {
    #[cfg(feature = "en")]
    pub use super::scanner::scan_tokens;
}
