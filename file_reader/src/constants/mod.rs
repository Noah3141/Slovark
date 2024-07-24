pub const INTERMEDIARY_DELIMITER: &'static str = "========================================";

pub mod page_labels {
    pub mod ru {
        ///
        pub const INFLECTED_FORM: &'static str = "{{infl";
        // ===Noun===
        // {{head|ru|noun form|head=ка́лии|g=m-in-p|g2=m-in}}
        // # {{inflection of|ru|ка́лий||nom//acc|p|;|pre|s}}
        ///
        pub const NONINFLECTED_FORM: &'static str = "{{ru-";
        // ====Conjugation====
        // {{ru-conj|pf|6c+p|оказа́ть}}
        // ====Declension====
        // {{ru-noun-table|Юпи́тер|a=ia}}
        // ===Noun===
        // {{ru-noun+|футбо́лка|*}}
    }

    pub mod ukr {
        ///
        pub const INFLECTED_FORM: &'static str = "{{infl";
        // ===Adjective===
        // {{head|uk|adjective form|head=регіона́льному}}
        // # {{infl of|uk|регіона́льний||m//n|dat|s|;|m//n|loc|s}}
        // ===Noun===
        // {{head|uk|noun form|head=дити́нстві|g=n-in}}
        // # {{infl of|uk|дити́нство||loc|s}}
        // ===Verb===
        // {{head|uk|verb form|head=відмо́вилися}}
        // # {{infl of|uk|відмо́витися||p|past|ind|g=pf}}
        // #: {{alti|uk|відмо́вились}}
        ///
        pub const NONINFLECTED_FORM: &'static str = "{{uk-";
        // ===Noun===
        // {{uk-noun|футбо́лка&lt;*&gt;}}
    }

    pub mod bela {
        ///
        pub const INFLECTED_FORM: &'static str = "{{infl";
        // ===Noun===
        // {{head|be|noun form|head=саба́к}}
        // # {{infl of|be|саба́ка||gen//acc|p}}
        // ===Noun===
        // {{head|be|noun form|head=во́чы}}
        // # {{inflection of|be|во́ка||nom//acc|p}}
        ///
        pub const NONINFLECTED_FORM: &'static str = "{{be-";
        // ===Noun===
        // {{be-noun|пёс&lt;b*.anml&gt;}}
        // ===Noun===
        // {{be-noun|ко́шка&lt;*.anml&gt;}}
        // ===Verb===
        // {{be-verb|пасябрава́ць|pf|impf=сябрава́ць}}
        // ===Verb===
        // {{be-verb|вы́ссаць|pf|impf=высыса́ць}} {{tlb|be|transitive}}
        // ===Adjective===
        // {{be-adj|іра́нскі}}
        // ===Adverb===
        // {{be-adv|сёлета}}
    }
}
