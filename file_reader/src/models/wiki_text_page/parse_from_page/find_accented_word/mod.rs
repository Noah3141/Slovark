use super::{InitError, WikiTextPage};
mod test;

impl WikiTextPage {
    /// 5 Finds the accented representation of the word, e.g. ко́шка. These are absent when single syllable, where the Russian spelling is proxied by just having the wikitext say "{{ru-IPA}}". In these cases, return None, and we'll set the accented = plain
    pub fn find_accented_word(russian_section: &str) -> Result<Option<String>, InitError> {
        // {{ru-IPA|эска́дренный бронено́сец}}
        let pronunciation_start: usize =
            russian_section
                .find("===Pronunciation===")
                .ok_or(InitError::MissingExtraPiece(
                    "No pronunciation section!".to_string(),
                ))?;
        let pronunciation: &str = &russian_section[pronunciation_start..];
        let start: usize = pronunciation.find("{{").ok_or(InitError::MissingCorePiece(
            "Open bracket to pronunciation missing!".to_string(),
        ))?;
        let end: usize = pronunciation.find("}}").ok_or(InitError::MissingCorePiece(
            "Closing bracket to pronunciation missing!".to_string(),
        ))?;

        let bracketed_word = pronunciation[start..end].to_string();
        let word = if bracketed_word.starts_with("{{ru-IPA|") {
            Some(bracketed_word["{{ru-IPA|".len()..].to_string())
        } else {
            return Ok(None);
        };
        Ok(word)
    }
}