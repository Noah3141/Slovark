use crate::models::wiki_text_macro::WikiTextMacro;



#[test]
fn extraction_of_lemma1() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{ru-noun+|b|[[жу́к]]|-|[[носоро́г]]|a=an}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"жу́к-носоро́г"#))
}

#[test]
fn extraction_of_lemma2() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{ru-noun+|b|a=an|f=жучи́ха|adj=жу́чий|adj2=жучи́ный|dim=жучо́к|aug=жучи́ще|aug2=жуча́ра|pej=жучи́шка}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#""#))
}

#[test]
fn extraction_of_lemma3() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{ru-noun+|b|мужи́к|a=an|adj=мужи́цкий|adj2=мужи́чий|dim=мужичо́к|aug=мужичи́ще|pej=мужичо́нка|pej2=мужичи́шка}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"мужи́к"#))
}

#[test]
fn extraction_of_lemma4() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{ru-noun+|[[сахарный|са́харная]]|+|_|[[ва́та]]}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"са́харная ва́та"#))
}

#[test]
fn extraction_of_lemma5() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{be-noun|феміні́зм&lt;sg&gt;}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"феміні́зм"#))
}

#[test]
fn extraction_of_lemma6() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{be-adj|безалкаго́льны}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"безалкаго́льны"#))
}


#[test]
fn extraction_of_lemma7() {
    let text_macro = WikiTextMacro {
        text: String::from(r#"{{head|uk|verb form|head=відмо́вилися}}

# {{infl of|uk|відмо́витися||p|past|ind|g=pf}}
#: {{alti|uk|відмо́вились}}"#)
    };

    let lemma = text_macro.extract_to_lemma();
    assert_eq!(lemma, String::from(r#"відмо́витися"#))
}