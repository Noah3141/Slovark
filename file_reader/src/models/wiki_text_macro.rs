use std::mem::replace;

fn remove_accents(input: &str) -> String {
    input
        .chars()
        .filter(|&c| !is_combining_character(c))
        .collect()
}

fn is_combining_character(c: char) -> bool {
    match c {
        '\u{0300}'..='\u{036F}'
        | '\u{1DC0}'..='\u{1DFF}'
        | '\u{20D0}'..='\u{20FF}'
        | '\u{FE20}'..='\u{FE2F}' => true,
        _ => false,
    }
}

pub struct NounMacro {
    ///```
    ///
    /// // {{ru-noun+|b|[[жу́к]]|-|[[носоро́г]]|a=an}}
    /// // {{ru-noun+|b|a=an|f=жучи́ха|adj=жу́чий|adj2=жучи́ный|dim=жучо́к|aug=жучи́ще|aug2=жуча́ра|pej=жучи́шка}}
    /// // {{ru-noun+|b|мужи́к|a=an|adj=мужи́цкий|adj2=мужи́чий|dim=мужичо́к|aug=мужичи́ще|pej=мужичо́нка|pej2=мужичи́шка}}
    /// // {{ru-noun+|[[сахарный|са́харная]]|+|_|[[ва́та]]}}
    /// ```
    text: String,
}

impl NounMacro {
    const NOUN_MACRO_START_LABEL: &'static str = "{{ru-noun+|";
    pub fn find_first_in(text: &str) -> Option<Self> {
        if text.contains(Self::NOUN_MACRO_START_LABEL) {
            let macro_start = text
                .find(Self::NOUN_MACRO_START_LABEL)
                .expect("NOUN_MACRO_START_LABEL presence");
            let start_onward = &text[macro_start..];
            let macro_end = start_onward
                .find("}}")
                .expect("closing brackets on suspected NOUN_MACRO");
            let noun_macro = &start_onward[..macro_end + "}}".len()];
            return Some(NounMacro {
                text: noun_macro.to_string(),
            });
        }

        None
    }

    pub fn core_noun_text(&self) -> Option<String> {
        // {{ru-noun+|[[сахарный|са́харная]]|+|_|[[ва́та]]}}
        // {{ru-noun+|     [[сахарный    |     са́харная]]     |    +  |  _   |   [[ва́та]]     }}

        let mut parts = self.text.split("|").collect::<Vec<&str>>();

        if self.text.matches("[[").count() > 1 {
            parts.retain(|part| {
                part.contains("[[") || part.contains("]]") || *part == "_" || *part == "-"
            });
            let phrase = parts
                .iter_mut()
                .map(|part| -> String {
                    if part.contains("[[") && !part.contains("]]") {
                        return "".to_owned();
                    }

                    part.replace("[[", "")
                        .replace("]]", "")
                        .replace("_", " ")
                        .replace("}}", "")
                })
                .reduce(|acc, part| acc + &part)
                .expect("reduction");

            return Some(phrase);
        }

        for part in parts {
            if !part.contains(|c| {
                [
                    'b', 'r', 'c', '}', '{', '(', ')', '=', 'f', 'e', 'o', 'd', 'm', 'a', '*',
                ]
                .contains(&c)
            }) && part != ""
            {
                return Some(part.to_string());
            } else {
                continue;
            }
        }

        None
    }
}
