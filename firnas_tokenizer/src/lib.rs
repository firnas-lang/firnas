#[cfg(feature = "en")]
pub mod scanner;

#[cfg(feature = "ar")]
pub mod ar_scanner;

pub mod error;
pub mod token;

pub mod tokenizer {
    #[cfg(feature = "en")]
    pub use super::scanner::scan_tokens;

    #[cfg(feature = "ar")]
    pub use super::ar_scanner::scan_tokens;
}
