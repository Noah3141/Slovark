use super::{InitError, WikiTextPage};
mod test;

impl WikiTextPage {
    const ID_START: &'static str = "<id>";
    const ID_END: &'static str = "</id>";

      /// 1
    pub fn find_id(page: &str) -> Result<u64, InitError> {
        let start: usize = page
            .find(Self::ID_START)
            .ok_or(InitError::MissingCorePiece(
                "No start id tag found.".to_string(),
            ))?
            + Self::ID_START.len();
        let end: usize = page.find(Self::ID_END).ok_or(InitError::MissingCorePiece(
            "No end id tag found.".to_string(),
        ))?;
        let id: u64 = page[start..end]
            .parse::<u64>()
            .map_err(|_| InitError::MissingCorePiece("Failed to parse as int.".to_string()))?;
        Ok(id)
    }
}