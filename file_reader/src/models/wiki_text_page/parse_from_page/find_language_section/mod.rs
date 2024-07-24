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
                    "==Uzbek==",
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
                let ukrainian_start = page.find("==Ukrainian==").expect("No russian section found!");
                let ukrainian_onward = &page[ukrainian_start..];
                let section_headers = [
                    "==Serbo-Croatian==",
                    "==Tabasaran==",
                    "==Uzbek==",
                    "==Udmurt==",
                    "==Southern Altai==",
                    "==Tatar==",
                    "==Tundra Nenets==",
                    "==Tuvan==",
                    "==Wakhi==",
                    "==Western Mari==",
                    "==Ubykh==",
                    "==Yakut==",
                    "==Zhuang=="
                ];

                let any_section_switch = section_headers
                    .iter() // Go through headers
                    .filter_map( // For each
                        |&header| { // Header
                            ukrainian_onward.find(header) // If you find it
                                .map(|i| i + ukrainian_start) // Get the index relative to our context
                        }
                    ) // Filter all the Nones where the header hadn't been found
                    .min() // Get the lowest index of any FOUND header if one was
                    .unwrap_or(page.len()); // Otherwise if no header was found, just use the index of the end of the page
                let ukrainian_section = &page[ukrainian_start..any_section_switch]; // Get me that given section (potentially just to the end of page)

                ukrainian_section
            }
            Language::Belarusian => {
                let belarusian_start = page.find("==Belarusian==").expect("No russian section found!");
                let belarusian_onward = &page[belarusian_start..];
                let section_headers = [
                    "==Bulgarian==",
                    "==Buryat==",
                    "==Chechen==",
                    "==Chukchi==",
                    "==Carpathian Rusyn==",
                ];

                let any_section_switch = section_headers
                    .iter() // Go through headers
                    .filter_map( // For each
                        |&header| { // Header
                            belarusian_onward.find(header) // If you find it
                                .map(|i| i + belarusian_start) // Get the index relative to our context
                        }
                    ) // Filter all the Nones where the header hadn't been found
                    .min() // Get the lowest index of any FOUND header if one was
                    .unwrap_or(page.len()); // Otherwise if no header was found, just use the index of the end of the page
                let belarusian_section = &page[belarusian_start..any_section_switch]; // Get me that given section (potentially just to the end of page)

                belarusian_section
            }
        }
    }
}