namespace Slovark;
using System.Text.Json.Nodes;
using Newtonsoft.Json;
using System.Globalization;
using System.IO;
using CsvHelper;
using CsvHelper.Configuration;


public class FrequencyTable
{
    public static readonly (string, int)[] RussianFrequencies = Init();


    private static (string, int)[] Init()
    {
        CsvReader csv = new(
            new StreamReader("./frequency_labeled_lemmas.csv"),
            new CsvConfiguration(CultureInfo.InvariantCulture)
        );

        (string, int)[] russianFrequencies = [];

        while (csv.Read())
        {
            russianFrequencies.Append(
                (csv.GetField<string>("lemma"), csv.GetField<int>("frequency"))
            );
        }

        return russianFrequencies;
    }




    public static int? LookUpCommonalityRussian(string lemma)
    {
        int entry = (
            from row in RussianFrequencies
            where row.Item1 == lemma
            select row.Item2
        ).FirstOrDefault(-1);

        if (entry < 0) return null;
        else return entry;
    }
}
