use crate::models::Language;
mod test;

use super::WikiTextPage;

impl WikiTextPage {
    /// 3
    pub fn find_language_section<'a>(language: &Language, page: &'a str) -> &'a str {
        // Now parse down the Russian section only via detecting any OTHER sections
        match language {
            Language::Russian => {
                let russian_start = page.find("==Russian==").expect("No russian section found!");
                let russian_onward = &page[russian_start..];
                let section_headers = [
                    "==Ukrainian==",
                    "==Serbo-Croatian==",
                    "==Tabasaran==",
                    "==Udmurt==",
                    "==Southern Altai==",
                    "==Tatar==",
                    "==Tundra Nenets==",
                    "==Tuvan==",
                    "==Wakhi==",
                    "==Western Mari==",
                    "==Ubykh==",
                    "==Yakut==",
                ];

                let any_section_switch = section_headers
                    .iter() // Go through headers
                    .filter_map( // For each
                        |&header| { // Header
                            russian_onward.find(header) // If you find it
                                .map(|i| i + russian_start) // Get the index relative to our context
                        }
                    ) // Filter all the Nones where the header hadn't been found
                    .min() // Get the lowest index of any FOUND header if one was
                    .unwrap_or(page.len()); // Otherwise if no header was found, just use the index of the end of the page
                let russian_section = &page[russian_start..any_section_switch]; // Get me that given section (potentially just to the end of page)

                russian_section
            }
            Language::Ukrainian => {
                panic!("Not implemented")
            }
            Language::Belarusian => {
                panic!("Not implemented")
            }
        }
    }
}