use crate::models::{wiki_text_macro::WikiTextMacro, Language};


#[test]
fn finding_first_wiki_text_macro1() {
    let wikimacro = WikiTextMacro::find_first(r#"==Belarusian==
{{wikipedia|lang=be}}

===Pronunciation===
* {{be-IPA|гараско́п}}

===Noun===
{{be-noun|гараско́п&lt;&gt;}}

# [[horoscope]] {{gloss|astrological forecast}}

====Declension====
{{be-ndecl|гараско́п&lt;&gt;}}

===References===
* {{R:be:slounik.org}}

{{topics|be|Astrology|Occult|Pseudoscience}}"#, &Language::Belarusian)
        .expect("Finding the macro contained");

    assert_eq!(wikimacro.text, r#"{{be-noun|гараско́п&lt;&gt;}}"#);
}

#[test]
fn finding_first_wiki_text_macro2() {
    let wikimacro = WikiTextMacro::find_first(r#"==Belarusian==
{{wp|lang=be}}

===Alternative forms===
* {{l|be|фэміні́зм}}

===Pronunciation===
* {{be-IPA|феміні́зм}}

===Noun===
{{be-noun|феміні́зм&lt;sg&gt;}}

# [[feminism]]

====Declension====
{{be-ndecl|феміні́зм&lt;sg&gt;}}

===Further reading===
* {{R:be:slounik.org}}

{{c|be|Feminism}}

"#, &Language::Belarusian)
        .expect("Finding the macro contained");

    assert_eq!(wikimacro.text, r#"{{be-noun|феміні́зм&lt;sg&gt;}}"#);
}


#[test]
fn finding_first_wiki_text_macro3() {
    let wikimacro = WikiTextMacro::find_first(r#"==Russian==

===Pronunciation===
* {{ru-IPA|перерабо́тками}}

===Noun===
{{head|ru|noun form|head=перерабо́тками|g=f-in-p}}

# {{inflection of|ru|перерабо́тка||ins|p}}</text>
      <sha1>r3pjf5v87y5dhjsp7penm2invkggfpo</sha1>
    </revision>
  </page>"#, &Language::Russian)
        .expect("Finding the macro contained");

    assert_eq!(wikimacro.text, r#"{{inflection of|ru|перерабо́тка||ins|p}}"#.to_string())
}


#[test]
fn finding_first_wiki_text_macro4() {
    let wikimacro = WikiTextMacro::find_first(r#"==Russian==

===Etymology===
Perhaps from {{der|ru|fr|neutraliser}} + {{af|ru|-ова́ть}}.

===Pronunciation===
* {{ru-IPA|нейтрализова́ть}}
* {{audio|ru|Ru-нейтрализовать.ogg}}

===Verb===
{{ru-verb|нейтрализова́ть|both}}

# to [[neutralize]]

====Conjugation====
''imperfective''
{{ru-conj|impf|2a+p|нейтрализова́ть}}
''perfective''
{{ru-conj|pf|2a+p|нейтрализова́ть}}

====Related terms====
* {{l|ru|нейтра́льный}}
* {{l|ru|нейтрализа́ция}}
* {{l|ru|нейтралите́т}}
* {{l|ru|нейтрализа́тор}}
* {{l|ru|нейтро́н}}</text>
      <sha1>mdys2y0h483p60aeybm5jqb5dhpqpx9</sha1>
    </revision>
  </page>"#, &Language::Russian)
        .expect("Finding the macro contained");

    assert_eq!(wikimacro.text, r#"{{ru-verb|нейтрализова́ть|both}}"#);
}


