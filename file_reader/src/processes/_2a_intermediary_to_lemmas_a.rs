use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{constants::INTERMEDIARY_DELIMITER, models::WikiTextPage, RUN_OPTS};

/// Takes an intermediary file and creates a form-lemma key. This is the first of potentially multiple steps importing data into this file. Others may draw from other sources, but this draws from WikiText inflected form existing pages, which are frequently non-comprehensive across forms, but is a large dataset.
///
pub fn run() -> Result<(), &'static str> {
    let mut intermediary_text = String::with_capacity(50_000);
    File::options()
        .create(true)
        .write(true)
        .open(RUN_OPTS.intermediary_out())
        .unwrap()
        .read_to_string(&mut intermediary_text)
        .expect("writing to variable to pass out of fn");

    let mut form_lemma_file = File::options()
        .create_new(true)
        .append(true)
        .open(RUN_OPTS.form_lemma_out())
        .expect("Open form_lemma out file");

    for page in intermediary_text.split(INTERMEDIARY_DELIMITER) {
        if page == "" { continue; }
        let extract_result = WikiTextPage::write_lemma_entry(page, false, &mut form_lemma_file);
    }
    // Create Form-Lemma sheet

    Ok(())
}

fn combine() {
    let mut main_data = String::with_capacity(300_000);
    File::open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\augmented_lemma_key.txt")
        .expect("Open source file")
        .read_to_string(&mut main_data)
        .expect("read to string");

    let mut augmenting_data = String::with_capacity(300_000);
    File::open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\word_database_key.txt")
        .expect("Open source file")
        .read_to_string(&mut augmenting_data)
        .expect("read to string");

    let mut output_file = File::options()
        .append(true)
        .create(true)
        .open("C:\\Users\\Noah3\\Code\\Slovark\\lemma_data\\comprehensive_lemma.csv")
        .expect("output handle");
    output_file
        .write(main_data.as_bytes())
        .expect("Write main data");

    let main_forms: Vec<&str> = main_data
        .split("\n")
        .map(|line| {
            // println!("{line}");
            let comma = line.find(",").expect("find comma delimiter");
            return &line[..comma];
        })
        .collect();

    let new_entries = augmenting_data.split_whitespace();
    // .map(|line| { &line[line.find(",").unwrap()..]});

    output_file.write(b"====").expect("divider");

    for new_entry in new_entries {
        let (new_form, new_lemma) = new_entry.split_once(",").expect("split");
        if !main_forms.contains(&new_form) {
            println!("main forms did not contain {new_form}");
            output_file
                .write(format!("{new_form},{new_lemma}\n").as_bytes())
                .expect("write");
        }
    }
}
