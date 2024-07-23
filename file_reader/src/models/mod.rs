pub mod wiki_text_page;

pub use wiki_text_page::WikiTextPage;
pub use wiki_text_page::WikiTextPageInitError;
pub mod pos;
pub mod wiki_api_response;
pub mod wiki_text_macro;

pub enum Language {
    Russian,
    Ukrainian,
    Belarusian,
}

impl Language {
    pub fn to_string(self) -> String {
        match self {
            Language::Russian => "Russian".to_string(),
            Language::Ukrainian => "Ukrainian".to_string(),
            Language::Belarusian => "Belarusian".to_string(),
        }
    }
}
