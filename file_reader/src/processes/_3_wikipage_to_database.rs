use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{models::WikiTextPage, RUN_OPTS};
use reqwest::{Client, Method};
use serde_json::json;

///
pub fn run() -> Result<(), String> {
    let wiki_page_json = File::open(RUN_OPTS.wiki_pages_out()).expect("Reading wikipage json file");
    let wiki_pages: Vec<WikiTextPage> =
        serde_json::from_reader(wiki_page_json).expect("Deserialization of wikipage json file");

    let mut full_wiki_pages_dump = File::options()
        .append(true)
        .open("C:\\Users\\Noah3\\Code\\Slovark\\entry_data\\complete_entries.csv")
        .expect("open file");

    for wiki_page in wiki_pages {
        let r#type = wiki_page.pos.to_string();
        let info = wiki_page.get_full_info();

        full_wiki_pages_dump
            .write(
                format!(
                    "{lemma},{commonality},{type},{dictionary_info}",
                    lemma = info.lemma,
                    commonality = {
                        match info.commonality {
                            Some(num) => num.to_string(),
                            None => "null".to_string(),
                        }
                    },
                    r#type = r#type,
                    dictionary_info = json!(info.dictionary_info)
                )
                .as_bytes(),
            )
            .expect("write");

        let client = reqwest::ClientBuilder::new()
            //
            .build()
            .expect("building client");

        let mut lemma_input_file = String::with_capacity(30_000);
        File::open("C:\\Users\\Noah3\\Code\\Slovark\\entry_data\\parsed_pages_all.txt")
            .expect("input file read")
            .read_to_string(&mut lemma_input_file)
            .expect("read to string");

        let wiki_pages: Vec<WikiTextPage> =
            serde_json::from_str(&lemma_input_file).expect("deserialization");

        let output_file = File::open("./").expect("msg");

        // let mut wiki_pages_text = File::options().create(true).write(true).open("C:\\Users\\Noah3\\Code\\Slovark\\entry_data\\parsed_pages_all.txt").expect("opening file");
        // let mut wiki_pages_binary = File::options().create(true).write(true).open("C:\\Users\\Noah3\\Code\\Slovark\\entry_data\\parsed_pages_all.bin").expect("opening file");

        // wiki_pages_text.write_all(serde_json::to_string_pretty(&wiki_pages).expect("serialization").as_bytes()).expect("writing");

        // bincode::serialize_into(wiki_pages_binary, &*wiki_pages.into_boxed_slice()).expect("serialization");
        let mut full_wiki_pages_dump = File::options()
            .append(true)
            .open("C:\\Users\\Noah3\\Code\\Slovark\\entry_data\\complete_entries.csv")
            .expect("open file");

        for wiki_page in wiki_pages {
            let r#type = wiki_page.pos.to_string();
            let info = wiki_page.get_full_info();

            full_wiki_pages_dump
                .write(
                    format!(
                        "{lemma},{commonality},{type},{dictionary_info}",
                        lemma = info.lemma,
                        commonality = {
                            match info.commonality {
                                Some(num) => num.to_string(),
                                None => "null".to_string(),
                            }
                        },
                        r#type = r#type,
                        dictionary_info = json!(info.dictionary_info)
                    )
                    .as_bytes(),
                )
                .expect("writing");
        }
    }

    Ok(())
}

// URL = "https://en.wiktionary.org/w/api.php"
// TITLE = "авуары"
// PARAMS = {
//     'action': "parse",
//     # 'page': TITLE,
//     'prop': 'text',
//     # 'section': 5,
//     'format': "json",
//     "text": """{{ru-noun-table|авуа́ры|m}}"""
// }

async fn expand_macro_request(
    client: &Client,
    macro_text: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    client
        .request(Method::GET, "https://en.wiktionary.org/w/api.php")
        .query(&[
            ("action", "parse"),
            ("prop", "text"),
            ("format", "json"),
            ("text", macro_text),
        ])
        .send()
        .await
}
