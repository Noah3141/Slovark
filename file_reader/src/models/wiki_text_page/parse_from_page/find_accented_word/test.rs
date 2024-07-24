use crate::models::WikiTextPage;

#[test]
fn test_find_accented_word() {

    assert_eq!(


    WikiTextPage::find_accented_word(r#"""=======================================================start

<page>
<title>полседьмого</title>
<ns>0</ns>
<id>5036007</id>
<revision>
<id>66991125</id>
<parentid>51816945</parentid>
<timestamp>2022-05-15T09:22:07Z</timestamp>
<contributor>
    <username>WingerBot</username>
    <id>2024159</id>
</contributor>
<minor />
<comment>use {{af}} in place of {{prefix}}/{{suffix}}/{{affix}} etc.; move {{wikipedia}} lines to top of etym section (automatic + manually assisted)</comment>
<origin>66991125</origin>
<model>wikitext</model>
<format>text/x-wiki</format>
<text bytes="512" sha1="k4unqvyy96azadadhxm08yjloz8lrpk" xml:space="preserve">==Russian==

===Etymology===
{{af|ru|пол-|седьмо́й|t1=[[half]]|t2=[[seventh]]}}, with {{m|ru|седьмо́й}} in the genitive case, as with other words prefixed with {{m|ru|пол-}}. Compare {{cog|de|halb sieben||half past six|lit=half seven}}.

===Pronunciation===
* {{ru-IPA|по̀лседьмо́го}}

===Noun===
{{ru-noun|полседьмо́го|tr=polsedʹmóvo|n-in|-}}

# {{lb|ru|colloquial}} [[half past]] [[six]], [[six-thirty]] {{gloss|hour of the day}}

====Declension====
Invariable.</text>
<sha1>k4unqvyy96azadadhxm08yjloz8lrpk</sha1>
</revision>
</page>

=======================================================end"""#).expect("to find it").expect("msg"),
        "по̀лседьмо́го".to_string()
)

}
