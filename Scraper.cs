namespace Slovark;

using HtmlAgilityPack;
using Slovark.Models;

public class Scraper
{
    private static readonly HttpClient client = new();

    public async Task<List<RussianEntry>> GetRussianEntry(string lemma)
    {
        List<RussianEntry> russianEntries = [];

        HtmlDocument wikiPage = await GetPage($"https://en.wiktionary.org/wiki/{lemma}");
        HtmlNode russianSection = wikiPage.DocumentNode.SelectSingleNode(
            "//*[@id='Russian']"
        ) ?? throw new Exception("No Russian section found in page");


        HtmlNode? verbSection = russianSection.SelectSingleNode(
            "//h3[text()='Verb']"
        );
        if (verbSection is not null)
        {
            RussianEntry.Verb russianVerb = ProcessVerbSection(verbSection);
            RussianEntry vocabEntry = RussianEntry.FromVerb(
                russianVerb,
                FrequencyTable.LookUpCommonalityRussian(lemma)
            );
            russianEntries.Add(vocabEntry);
        }

        HtmlNode? nounSection = russianSection.SelectSingleNode(
            "//h3[text()='Noun']"
        );
        if (nounSection is not null)
        {
            RussianEntry.Noun russianNoun = ProcessNounSection(nounSection);
            RussianEntry vocabEntry = RussianEntry.FromNoun(
                russianNoun,
                FrequencyTable.LookUpCommonalityRussian(lemma)
            );
            russianEntries.Add(vocabEntry);
        }

        HtmlNode? adjectiveSection = russianSection.SelectSingleNode(
            "//h3[text()='Noun']"
        );
        if (adjectiveSection is not null)
        {
            RussianEntry.Adjective russianAdjective = ProcessAdjectiveSection(nounSection);
            RussianEntry vocabEntry = RussianEntry.FromAdjective(
                russianAdjective,
                FrequencyTable.LookUpCommonalityRussian(lemma)
            );
            russianEntries.Add(vocabEntry);
        }

        return russianEntries;
    }

    //
    //
    //
    private string FindTagTextByClass(HtmlDocument html, string classToken)
    {
        var node = html.DocumentNode.SelectSingleNode(
            $"//*[@class='{classToken}']"
        ) ?? throw new Exception("Could not find specified node in HTML");
        return node.InnerText.Trim();
    }

    //
    private async Task<HtmlDocument> GetPage(string url)
    {
        HttpResponseMessage response = await client.GetAsync(url);
        response.EnsureSuccessStatusCode();
        string responseBody = await response.Content.ReadAsStringAsync();
        var doc = new HtmlDocument();
        doc.LoadHtml(responseBody);
        return doc;
    }

    //
    private RussianEntry.Verb ProcessVerbSection(HtmlNode section)
    {


        return new RussianEntry.Verb
        {
            
        }
    }

    private RussianEntry.Noun ProcessNounSection(HtmlNode section)
    {

    }

    private RussianEntry.Adjective ProcessAdjectiveSection(HtmlNode section)
    {

    }
}
