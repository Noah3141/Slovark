use crate::models::WikiTextPage;

#[test]
fn extraction_of_form_lemma() {

    assert_eq!( 
        {match WikiTextPage::extract_lemma_entry(r#"===================================================
<page>
<title>сделай</title>
<ns>0</ns>
<id>5149302</id>
<revision>
    <id>57112171</id>
    <parentid>36405946</parentid>
    <timestamp>2019-10-19T11:36:00Z</timestamp>
    <contributor>
    <username>WingerBot</username>
    <id>2024159</id>
    </contributor>
    <minor />
    move lang= to 1= in {{inflection of}}</comment>
    <origin>57112171</origin>
    <model>wikitext</model>
    <format>text/x-wiki</format>
    <text bytes="168" sha1="7891jiyat6wj10vh1jbsh807xe1hont" xml:space="preserve">==Russian==

===Pronunciation===
* {{ru-IPA|сде́лай}}

===Verb===
{{head|ru|verb form|head=сде́лай}}

# {{inflection of|ru|сде́лать||2|s|imp|pfv}}</text>
    <sha1>7891jiyat6wj10vh1jbsh807xe1hont</sha1>
</revision>
</page>
==================================================="#, false) {
            Ok(d) => d,
            Err(e) => panic!("{e}")
        }}, (String::from("сделай"), String::from("сде́лать"))
    );
}