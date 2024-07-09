
"""
    parse_wikitable.py

    MediaWiki Action API Code Samples
    Demo of `Parse` module: Parse a section of a page, fetch its table data and save
    it to a CSV file

    MIT license


<page>
    <title>авуары</title>
    <ns>0</ns>
    <id>5861276</id>
    <revision>
      <id>65049916</id>
      <parentid>65022991</parentid>
      <timestamp>2021-12-26T22:32:07Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      <comment>use {{syn}}/{{ant}}/{{bor+}}/{{inh+}}/{{alt}} for Russian lemmas; misc cleanups (manually assisted)</comment>
      <origin>65049916</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="310" sha1="1d93hhnodwik1tprie12ich9fw3j8il" xml:space="preserve">==Russian==

===Etymology===
{{bor+|ru|fr|avoirs}}, plural of {{m|fr|avoir}}.

===Pronunciation===
* {{ru-IPA|авуа́ры}}

===Noun===
{{ru-noun+|авуа́ры|m}}

# {{lb|ru|literary|or|stilted}} [[assets]], [[holdings]]
#: {{syn|ru|акти́вы}}

====Declension====
{{ru-noun-table|авуа́ры|m}}</text>
      <sha1>1d93hhnodwik1tprie12ich9fw3j8il</sha1>
    </revision>
  </page>

"""

import csv
import requests

S = requests.Session()

URL = "https://en.wiktionary.org/w/api.php"

TITLE = "авуары"

PARAMS = {
    'action': "parse",
    # 'page': TITLE,
    'prop': 'text',
    # 'section': 5,
    'format': "json",
    "text": """{{ru-noun-table|авуа́ры|m}}"""
}


def get_table():
    """ Parse a section of a page, fetch its table data and save it to a CSV file
    """
    res = S.get(url=URL, params=PARAMS)
    data = res.json()
    try:
        wikitext = data['parse']['wikitext']['*']
    except:
        print("\n\nFAIL")
        print(data)
        return
    
    print(data)
    lines = wikitext.split('|-')
    entries = []

    for line in lines:
        line = line.strip()
        if line.startswith("|"):
            table = line[2:].split('||')
            entry = table[0].split("|")[0].strip("'''[[]]\n"), table[0].split("|")[1].strip("\n")
            entries.append(entry)

    file = open("авуары.csv", "w")
    writer = csv.writer(file)
    writer.writerows(entries)
    file.close()

if __name__ == '__main__':
    get_table()