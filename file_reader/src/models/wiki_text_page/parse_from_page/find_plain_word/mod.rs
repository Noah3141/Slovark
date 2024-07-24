mod test;
use super::{InitError, WikiTextPage};

impl WikiTextPage {
    /// 2
    pub fn find_plain_word(page: &str) -> Result<String, InitError> {
        let start = page
            .find(Self::TITLE_START)
            .ok_or(InitError::MissingCorePiece(
                "No start title tag found.".to_string(),
            ))?
            + Self::TITLE_START.len();
        let end = page
            .find(Self::TITLE_END)
            .ok_or(InitError::MissingCorePiece(
                "No end title tag found.".to_string(),
            ))?;
        let title = page[start..end].to_string();

        if title.starts_with("Wiktionary:") || title.starts_with("User:") {
            return Err(InitError::NotADictionaryPage(page.to_string()));
        }
        Ok(title)
    }
}