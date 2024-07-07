using System.Text.Json.Nodes;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;



namespace Slovark.Models;

/// <summary>
/// 
/// </summary>
public class RussianEntry
{
    public string Lemma;
    public int? Commonality;
    public PartOfSpeech Type;
    public JObject Data; // Make this field JSON



    public enum PartOfSpeech
    {
        Noun,
        Verb,
        Adjective
    }

    public static RussianEntry FromVerb(RussianEntry.Verb verb, int? commonality)
    {
        return new RussianEntry
        {
            Lemma = verb.DictionaryForm,
            Commonality = commonality,
            Type = PartOfSpeech.Verb,
            Data = JObject.FromObject(verb)
        };
    }
    public static RussianEntry FromNoun(RussianEntry.Noun noun, int? commonality)
    {
        return new RussianEntry
        {
            Lemma = noun.DictionaryForm,
            Commonality = commonality,
            Type = PartOfSpeech.Noun,
            Data = JObject.FromObject(noun)
        };
    }
    public static RussianEntry FromAdjective(RussianEntry.Adjective adj, int? commonality)
    {
        return new RussianEntry
        {
            Lemma = adj.DictionaryForm,
            Commonality = commonality,
            Type = PartOfSpeech.Adjective,
            Data = JObject.FromObject(adj)
        };
    }


    /// <summary>
    ///     Noun
    /// </summary>
    public class Noun
    {
        public string IPA;
        public string NomSing;
        public string NomPlur;
        public string GenSing;
        public string GenPlur;
        public string DatSing;
        public string DatPlur;
        public string InsSing;
        public string InsPlur;
        public string PreSing;
        public string PrePlur;
        public string AccSing;
        public string AccPlur;
        public string DictionaryForm;

        public Noun(
            string nomSing,
            string nomPlur,
            string accSing,
            string accPlur,
            string genSing,
            string genPlur,
            string datSing,
            string datPlur,
            string insSing,
            string insPlur,
            string preSing,
            string prePlur
        )
        {
            NomSing = nomSing;
            NomSing = nomSing;
            NomPlur = nomPlur;
            GenSing = genSing;
            GenPlur = genPlur;
            DatSing = datSing;
            DatPlur = datPlur;
            InsSing = insSing;
            InsPlur = insPlur;
            PreSing = preSing;
            PrePlur = prePlur;
            AccSing = accSing;
            AccPlur = accPlur;
            DictionaryForm = NomSing;
        }
    }

    /// <summary>
    ///     Verb
    /// </summary>
    public class Verb
    {
        public string DictionaryForm;
        public string IPA;
        public bool Imperfective;
        public bool Perfective;
        public string? RootSpace;
        public string AspectualPair;
        public string ЯForm;
        public string ТыForm;
        public string ОнForm;
        public string МыForm;
        public string ВыForm;
        public string ОниForm;
        public string femPast;
        public string mascPast;
        public string neutPast;
        public string plurPast;
        public string? PresentActive;
        public string? PresentPassive;
        public string? PastPassive;
        public string? PastActive;
        public string? PresentAdverbial;
        public string? PastAdverbial;
        public string SingularImperative;
        public string PluralImperative;
        public bool Transitive;
        public bool Intransitive;

        public Verb()
        {

        }
    }


    /// <summary>
    ///     Adjective
    /// </summary>
    public class Adjective
    {
        public string IPA;

        public string NomMasc;
        public string AccMasc;
        public string GenMasc;
        public string DatMasc;
        public string InsMasc;
        public string PreMasc;

        public string NomFem;
        public string AccFem;
        public string GenFem;
        public string DatFem;
        public string InsFem;
        public string PreFem;

        public string NomNeut;
        public string AccNeut;
        public string GenNeut;
        public string DatNeut;
        public string InsNeut;
        public string PreNeut;

        public string NomPlur;
        public string AccPlur;
        public string GenPlur;
        public string DatPlur;
        public string InsPlur;
        public string PrePlur;

        public string DictionaryForm;
    }



}
