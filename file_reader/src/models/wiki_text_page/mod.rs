use serde::{Deserialize, Serialize};
use super::pos::PoS;

mod reqwest_info;
mod parse_from_page;
mod extracting_form_lemma;
pub mod error;

#[derive(Debug, Serialize, Deserialize)]
/// Faithful parsing of the WikiText strings in a page. Holds verbs, nouns, and adjective entries alike
pub struct WikiTextPage {
    pub page_id: u64,
    pub pos: PoS,
    pub plain_word: String,
    pub accented_word: String,
    pub table: String,
    pub etymology: Option<String>,
    pub related_terms: Option<String>,
}
