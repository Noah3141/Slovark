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

        from functools import partial
        from typing import Any

        from wikitextprocessor import Wtp, WikiNode, NodeKind, Page
        
        from wikitextprocessor.dumpparser import process_dump

        wtp = Wtp(
            db_path=".\\wtp_dbs\\en_20230801.db", lang_code="en", project="wiktionary"
        )

        

        process_dump(  # extract dump file then save pages to SQLite file
            wtp,
            ".\\core_wiki_dumps\\en_wiktionary.xml.bz2",
            {0, 10, 110, 828},  # namespace id, can be found at the start of dump file
            skip_extract_dump=True,
        )



        #
        #
        #

        def page_handler(wtp: Wtp, page: Page) -> Any:
            wtp.start_page(page.title)

            if not page.body: return;

            tree = wtp.parse(
                page.body, 
                expand_all=True,
                pre_expand=True
            )
            text = wtp.expand(
                page.body, 
                pre_expand=True,
                expand_invoke=True
            )

            


            print(text)


        for _ in map(
            partial(page_handler, wtp), wtp.get_all_pages([0])
        ):
            print(f"\n\n");
            pass

        pass




Program.main()