from LemmatizerDb import LemmatizerDb
import pandas as pd

class Program:

    @staticmethod
    def label_freq():
        with open("./russian-master/50000-russian-words-cyrillic-only.txt", "r", encoding="utf-8") as f:
            lines = f.readlines();
        

        df = pd.DataFrame({'lemma': [], 'frequency': []})
        for i, line in enumerate(lines):
            df.loc[len(df)] = {"lemma": line.strip(), "frequency": i}

            foo = df.filter()
            pass

        df.to_csv("./frequency_labeled_lemmas.csv");

    @staticmethod
    def main() -> None:

        # Take alllllll the lemma forms

        # Loop through them

        # Go to the internet and get Wiktionary Data

        # Parse response and create corresponding Class

        # Crunch that shit into a DictionaryEntry class and append it to the master list

        # Save that shit as a CSV

        pass


Program.main()