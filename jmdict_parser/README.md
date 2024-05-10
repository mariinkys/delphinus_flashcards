# Parser for the JMDict Dictionary

This program parses the JMDict Japanese Dictionary and converts it to a format that is more easily manageable for my intents and purposes.
The JMDict Japanese Dictionary original file can be downloaded from <a href="https://www.edrdg.org/jmdict/j_jmdict.html" target="_blank">HERE</a>.

The application expects a 'JMdict.xml' file in the root directory, it will generate a new .xml file removing some conflicting characters and then generate a .txt file containing the expected output file.

## Note 

This program uses the 'current version (Unicode UTF8) (Warning! This is a big file.)' not the one that only has the English translations.

The original file can be downloaded with curl as follows:

```curl -O ftp://ftp.edrdg.org/pub/Nihongo//JMdict.gz```

## Acknowledgement for a Software Package

This package uses the JMdict/EDICT and KANJIDIC dictionary files. These files are the property of the Electronic Dictionary Research and Development Group, and are used in conformance with the Group's license. 